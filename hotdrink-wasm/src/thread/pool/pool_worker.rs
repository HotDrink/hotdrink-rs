//! A wrapper around [`GenericWorker`] to make it more convenient to use in a thread pool.

use crate::thread::{worker::generic_worker::GenericWorker, TerminationStrategy};
use js_sys::Date;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Receiver,
    Arc, Mutex,
};
use wasm_bindgen::JsValue;

/// A representation of work.
/// It includes a function to execute,
/// as well as a flag that will be set
/// if the result is no longer required.
pub struct Work {
    work: Box<dyn FnOnce() + Send + 'static>,
    result_needed: Arc<AtomicBool>,
}

impl std::fmt::Debug for Work {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Work")
            .field("work", &"...")
            .field("result_needed", &self.result_needed.load(Ordering::SeqCst))
            .finish()
    }
}

impl Work {
    /// Create a new `Work` struct by passing in the function to execute, and a flag
    /// that is set if the result is no longer required.
    pub fn new(work: impl FnOnce() + Send + 'static, result_needed: Arc<AtomicBool>) -> Self {
        Work {
            work: Box::new(work),
            result_needed,
        }
    }
}

/// Data to be shared with and the main thread and other workers.
struct SharedData {
    /// The receiving end of the channel to receive work on.
    receiver: Arc<Mutex<Receiver<Work>>>,
    /// Whether or not the current task's result is unused.
    result_needed: Option<Arc<AtomicBool>>,
    /// A flag to be set if this thread has been terminated.
    is_terminated: Arc<AtomicBool>,
    /// A flag set when all workers should terminate.
    terminate_all: Arc<AtomicBool>,
    /// Start time
    start_time: Option<f64>,
}

impl SharedData {
    /// Sets the result_needed flag and the task start time.
    pub fn start_task(&mut self, result_needed: Arc<AtomicBool>) {
        self.start_time = Some(Date::now());
        self.result_needed = Some(result_needed);
    }

    /// Deletes the current information.
    pub fn end_task(&mut self) {
        self.start_time = None;
        self.result_needed = None;
    }

    /// Checks if the computation is currently active.
    pub fn is_active(&self) -> bool {
        self.result_needed.is_some()
    }

    /// Checks if the computation has been terminated.
    pub fn is_terminated(&self) -> bool {
        self.is_terminated.load(Ordering::SeqCst) || self.terminate_all.load(Ordering::SeqCst)
    }

    /// Checks if the internal flag is set, meaning the result is no longer required.
    pub fn result_needed(&self) -> bool {
        if let Some(result_needed) = &self.result_needed {
            if result_needed.load(Ordering::SeqCst) {
                return true;
            }
        }
        false
    }
}

/// A worker to be used in `StaticPool`.
pub struct PoolWorker {
    /// The worker's id.
    id: usize,
    /// The internal web worker.
    worker: GenericWorker,
    /// Data shared with other threads.
    shared_data: Arc<Mutex<SharedData>>,
    /// A flag set when all workers should terminate.
    terminate_all: Arc<AtomicBool>,
}

impl std::fmt::Debug for PoolWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolWorker")
            .field("id", &self.id)
            .field("worker", &self.worker)
            .field("shared_data", &"...")
            .field("terminate_all", &self.terminate_all.load(Ordering::SeqCst))
            .finish()
    }
}

impl PoolWorker {
    /// Tries to create a new `PoolWorker`.
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Work>>>,
        wasm_bindgen_shim_url: &str,
    ) -> Result<Self, JsValue> {
        // Create shared data
        let terminate_all = Arc::new(AtomicBool::new(false));
        let shared_data = Arc::new(Mutex::new(SharedData {
            receiver,
            result_needed: None,
            is_terminated: Arc::new(AtomicBool::new(false)),
            terminate_all: terminate_all.clone(),
            start_time: None,
        }));

        // Spawn new worker, and pass in shared fields
        let worker = Self::spawn(id, wasm_bindgen_shim_url, shared_data.clone())?;

        // Construct new worker
        let pw = Self {
            id,
            worker,
            shared_data,
            terminate_all,
        };
        Ok(pw)
    }

    /// Tries to spawn a new worker, and starts its work loop.
    fn spawn(
        id: usize,
        wasm_bindgen_shim_url: &str,
        shared_data: Arc<Mutex<SharedData>>,
    ) -> Result<GenericWorker, JsValue> {
        // Create new worker and shared fields
        let worker = GenericWorker::from_url(&format!("PoolWorker {}", id), wasm_bindgen_shim_url)?;
        // Start the work loop
        worker.execute(Box::new(move |_| loop {
            let work = {
                // Lock access to shared fields
                let mut shared_data = shared_data.lock().expect("Could not lock shared data");

                // Worker is terminated, abort work before locking receiver
                if shared_data.is_terminated() {
                    break;
                }

                // Receive a new task
                let Work {
                    work,
                    result_needed,
                } = shared_data
                    .receiver
                    .lock()
                    .expect("Could not lock receiver")
                    .recv()
                    .expect("Could not receive task");

                // Set task information to signify that task has started
                shared_data.start_task(result_needed);

                // Task result is already dropped, just skip to next task.
                // TODO: This means that intermediate results may be skipped, do we want that?
                if !shared_data.result_needed() {
                    continue;
                }

                work
            };

            // Execute work
            work();

            // Remove the old shared data to signify that the task is done
            shared_data.lock().unwrap().end_task();
        }))?;

        Ok(worker)
    }

    /// Completely restarts the worker if it appears to be stale.
    /// That is, if its result is dropped, but it is still working.
    /// TODO: Include a timer here as well to only stop longer-running tasks.
    pub fn restart_if_stale(
        &mut self,
        termination_strategy: &TerminationStrategy,
        wasm_bindgen_shim_url: &str,
    ) -> Result<(), JsValue> {
        // A variable to store a potential new worker in
        let mut new = None;

        // Try to lock shared data.
        // If it is already locked, then the worker is not stuck, and we can continue without issue.
        if let Ok(shared_data) = self.shared_data.try_lock() {
            let should_be_terminated = match termination_strategy {
                TerminationStrategy::Never => false,
                TerminationStrategy::UnusedResultAndNotDone => {
                    !shared_data.result_needed() && shared_data.is_active()
                }
                TerminationStrategy::UnusedResultAndNotDoneInMs(ms) => {
                    // Whether or not the task as been active for too long
                    let active_too_long = shared_data
                        .start_time
                        .map(|st| Date::now() - st > *ms as f64)
                        .unwrap_or(false);
                    !shared_data.result_needed() && shared_data.is_active() && active_too_long
                }
            };
            // Check if the task is dropped, and that the worker is still working on the task
            if should_be_terminated {
                log::trace!("Worker was terminated");
                // Terminate the worker
                shared_data.is_terminated.store(true, Ordering::SeqCst);
                self.worker.terminate();
                // Construct the new one
                new = Some(Self::new(
                    self.id,
                    shared_data.receiver.clone(),
                    wasm_bindgen_shim_url,
                )?);
            }
        }

        // Update self to the new worker if restarted
        if let Some(new) = new {
            *self = new;
        }

        Ok(())
    }

    /// Sets the termination flag, and terminates the internal worker.
    pub fn terminate(&self) {
        self.terminate_all.store(true, Ordering::SeqCst);
        self.worker.terminate();
    }
}
