//! A thread pool implementation that changes its number of workers dynamically depending on need.

use crate::thread::{worker::generic_worker::GenericWorker, TerminationStrategy};
use hotdrink_rs::thread::{MethodExecutor, TerminationHandle};
use js_sys::Date;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc, Mutex,
};
use wasm_bindgen::JsValue;

use super::WorkerPool;

/// Work to be passed to a worker.
struct Work {
    /// A function for the worker to execute.
    work: Box<dyn FnOnce() + Send + 'static>,
}

/// Information about a worker.
#[derive(Debug)]
struct WorkerInfo {
    /// The worker itself.
    worker: GenericWorker,
    /// A flag that will be set when the result of the
    /// current computation is no longer required.
    result_needed: Arc<AtomicBool>,
    /// A flag that is set after the computation is complete.
    ready: Arc<AtomicBool>,
    /// The sender-half of a channel that sends work to the worker.
    sender: Sender<Work>,
    /// The time when the task was started
    time_started: Arc<Mutex<Option<f64>>>,
}

impl WorkerInfo {
    /// Passes the work to be executed into the work-channel.
    pub fn execute(&mut self, f: impl FnOnce() + Send + 'static) {
        self.ready.store(false, Ordering::SeqCst);
        self.result_needed = Arc::new(AtomicBool::new(true));
        self.sender.send(Work { work: Box::new(f) }).unwrap();
    }

    /// Checks if the termination flag has been set,
    /// meaning that the result of the current computation
    /// likely has been dropped.
    pub fn result_needed(&self) -> bool {
        self.result_needed.load(Ordering::SeqCst)
    }

    /// Checks if the current computation has been completed.
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::SeqCst)
    }
}

/// Creates a new worker and starts it, making it receive and execute work in a loop.
fn spawn_worker(wasm_bindgen_shim_url: &str) -> Result<WorkerInfo, JsValue> {
    let worker = GenericWorker::from_url("PoolWorker", wasm_bindgen_shim_url)?;

    // Set up the shared data
    let ready = Arc::new(AtomicBool::new(true));
    let ready_clone = ready.clone();
    let time_started = Arc::new(Mutex::new(None));
    let time_started_clone = time_started.clone();
    let (sender, receiver) = std::sync::mpsc::channel::<Work>();
    // Start the worker loop, where it receives work, completes it, and then waits for more
    worker.execute(Box::new(move |_| loop {
        match receiver.recv() {
            Ok(task) => {
                *time_started_clone.lock().unwrap() = Some(Date::now());
                (task.work)();
                *time_started_clone.lock().unwrap() = None;
                ready_clone.store(true, Ordering::SeqCst);
            }
            Err(e) => {
                log::error!("Could not receive work: {}", e);
                break;
            }
        }
    }))?;

    Ok(WorkerInfo {
        worker,
        result_needed: Arc::new(AtomicBool::new(true)),
        ready,
        sender,
        time_started,
    })
}

/// A worker pool that automatically resizes itself when more workers are required.
#[derive(Debug)]
pub struct DynamicPool {
    workers: Vec<WorkerInfo>,
    termination_strategy: TerminationStrategy,
    wasm_bindgen_shim_url: String,
}

impl MethodExecutor for DynamicPool {
    type NewError = JsValue;
    type ExecError = JsValue;

    fn new(initial: usize) -> Result<Self, Self::NewError>
    where
        Self: Sized,
    {
        WorkerPool::from_url(
            initial,
            TerminationStrategy::UnusedResultAndNotDone,
            &crate::thread::worker::worker_script::create(),
        )
    }

    fn execute(
        &mut self,
        f: impl FnOnce() + Send + 'static,
    ) -> Result<TerminationHandle, Self::ExecError> {
        let termination_strategy = self.termination_strategy;
        // Cancel and remove stale workers
        self.workers.drain_filter(|w| {
            let should_be_terminated = match termination_strategy {
                TerminationStrategy::Never => false,
                TerminationStrategy::UnusedResultAndNotDone => !w.result_needed() && !w.is_ready(),
                TerminationStrategy::UnusedResultAndNotDoneInMs(ms) => {
                    let not_done_in_ms = w
                        .time_started
                        .lock()
                        .unwrap()
                        .map(|tss| Date::now() - tss > ms as f64)
                        .unwrap_or(false);
                    !w.result_needed() && !w.is_ready() && not_done_in_ms
                }
            };
            if should_be_terminated {
                w.worker.terminate();
                log::trace!("Terminated a thread");
            }
            should_be_terminated
        });

        // Check if any workers are ready, and spawn a new one if not
        let any_ready = self.workers.iter().any(|w| w.is_ready());
        if !any_ready {
            let worker_info = spawn_worker(&self.wasm_bindgen_shim_url)?;
            self.workers.push(worker_info);
        }

        // Execute task on a ready worker.
        // We know that one must be ready, as we spawned a new one if none were.
        let (th, result_needed) = TerminationHandle::new();
        for worker_info in &mut self.workers {
            if worker_info.is_ready() {
                worker_info.execute(f);
                worker_info.result_needed = result_needed;
                break;
            }
        }

        Ok(th)
    }
}

impl WorkerPool for DynamicPool {
    fn from_url(
        initial: usize,
        termination_strategy: TerminationStrategy,
        wasm_bindgen_shim_url: &str,
    ) -> Result<Self, Self::NewError>
    where
        Self: Sized,
    {
        // Initialize workers
        let mut workers = Vec::new();
        for _ in 0..initial {
            let worker = spawn_worker(wasm_bindgen_shim_url)?;
            workers.push(worker);
        }

        Ok(Self {
            workers,
            termination_strategy,
            wasm_bindgen_shim_url: wasm_bindgen_shim_url.to_owned(),
        })
    }
}
