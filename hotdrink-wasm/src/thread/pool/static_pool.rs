//! A thread pool implementation that has a static number of workers, but can restart any of them should they become stuck,
//! or if their results are no longer required.

use super::{pool_worker::Work, PoolWorker, WebWorkerPool};
use crate::thread::TerminationStrategy;
use hotdrink_rs::executor::{MethodExecutor, TerminationHandle};
use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// A worker pool with a static number of workers.
#[wasm_bindgen]
#[derive(Debug)]
pub struct StaticPool {
    workers: Vec<PoolWorker>,
    work_sender: Sender<Work>,
    worker_script_url: String,
    termination_strategy: TerminationStrategy,
}

impl MethodExecutor for StaticPool {
    type ExecError = JsValue;

    /// Sends the work through a channel to be executed by the first available thread.
    /// It will also restart threads that appear to be stuck if their result is no longer requied.
    fn schedule(
        &mut self,
        f: impl FnOnce() + Send + 'static,
    ) -> Result<TerminationHandle, JsValue> {
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

impl WebWorkerPool for StaticPool {
    type FromUrlError = JsValue;
    /// Tries to create a new [`StaticPool`] with the specified number of workers,
    /// and spawns them using the provided script url.
    fn from_url(
        initial: usize,
        termination_strategy: TerminationStrategy,
        worker_script_url: &str,
    ) -> Result<Self, Self::FromUrlError> {
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
