use super::{pool_worker::Work, PoolWorker};
use hotdrink_rs::thread::thread_pool::{
    TerminationHandle, TerminationStrategy, ThreadPool, WorkerPool,
};
use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// A worker pool with a static number of workers.
#[wasm_bindgen]
pub struct StaticPool {
    workers: Vec<PoolWorker>,
    work_sender: Sender<Work>,
    worker_script_url: String,
    termination_strategy: TerminationStrategy,
}

impl ThreadPool for StaticPool {
    type NewError = JsValue;
    type ExecError = JsValue;

    /// Tries to create a new `StaticPool` with the specified number of workers.
    fn new(initial: usize, termination_strategy: TerminationStrategy) -> Result<Self, JsValue> {
        Self::from_url(
            initial,
            termination_strategy,
            &crate::thread::worker::worker_script::create(),
        )
    }

    /// Sends the work through a channel to be executed by the first available thread.
    /// It will also restart threads that appear to be stuck if their result is no longer requied.
    fn execute(&mut self, f: impl FnOnce() + Send + 'static) -> Result<TerminationHandle, JsValue> {
        // Replace workers that will not produce a useful result
        for pw in &mut self.workers {
            pw.restart_if_stale(&self.termination_strategy, &self.worker_script_url)?;
        }

        // Insert the task into the work channel
        let (th, result_needed) = TerminationHandle::new();
        let work = Work::new(f, result_needed);
        self.work_sender.send(work).unwrap();

        // Return a handle that will set a flag once it is dropped
        Ok(th)
    }
}

impl WorkerPool for StaticPool {
    /// Tries to create a new `StaticPool` with the specified number of workers,
    /// and spawns them using the provided script url.
    fn from_url(
        initial: usize,
        termination_strategy: TerminationStrategy,
        worker_script_url: &str,
    ) -> Result<Self, JsValue> {
        // Create channels to communicate with workers through
        let (work_sender, work_receiver) = mpsc::channel();
        let work_receiver = Arc::new(Mutex::new(work_receiver));

        // Initialize workers
        let mut workers = Vec::new();
        for id in 0..initial {
            let worker = PoolWorker::new(id, work_receiver.clone(), worker_script_url)?;
            workers.push(worker);
        }

        // Initialize struct
        Ok(Self {
            workers,
            work_sender,
            worker_script_url: worker_script_url.to_owned(),
            termination_strategy,
        })
    }
}

impl Drop for StaticPool {
    /// Terminates all workers in the pool.
    fn drop(&mut self) {
        for worker in &self.workers {
            worker.terminate();
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables, clippy::mutex_atomic)]

    use hotdrink_rs::thread::thread_pool::TerminationHandle;
    use std::sync::atomic::Ordering;

    #[test]
    pub fn termination_handle_does_not_set_flag_while_in_scope() {
        let (th, flag) = TerminationHandle::new();
        assert_eq!(flag.load(Ordering::SeqCst), true);
    }

    #[test]
    pub fn termination_handle_sets_flag_when_out_of_scope() {
        let flag = {
            let (th, flag) = TerminationHandle::new();
            flag
        };
        assert_eq!(flag.load(Ordering::SeqCst), false);
    }

    #[test]
    pub fn termination_handle_does_not_set_flag_until_all_clones_out_of_scope() {
        let flag = {
            let (th1, flag) = TerminationHandle::new();
            {
                #[allow(clippy::redundant_clone)]
                let th2 = th1.clone();
                assert_eq!(flag.load(Ordering::SeqCst), true);
            }
            assert_eq!(flag.load(Ordering::SeqCst), true);
            flag
        };
        assert_eq!(flag.load(Ordering::SeqCst), false);
    }
}
