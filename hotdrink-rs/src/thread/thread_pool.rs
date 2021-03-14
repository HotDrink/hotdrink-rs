use std::fmt::Debug;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Three possibly strategies for when to terminate workers.
#[derive(Clone, Copy)]
pub enum TerminationStrategy {
    /// Never terminate any threads.
    Never,
    /// Terminate workers that compute a result that is no longer required,
    /// and are still not done with their current computation.
    UnusedResultAndNotDone,
    /// Terminate workers that compute a result that is no longer required,
    /// are still not done with their current computation,
    /// and have been working for the specified number of milliseconds.
    UnusedResultAndNotDoneInMs(usize),
}

/// A trait for thread pool implementations.
pub trait ThreadPool {
    type NewError: Debug;
    type ExecError: Debug;

    /// Creates a new thread pool with the specified number of initial workers.
    fn new(
        initial: usize,
        termination_strategy: TerminationStrategy,
    ) -> Result<Self, Self::NewError>
    where
        Self: Sized;

    /// Executes some work using the workers in the thread pool.
    fn execute(
        &mut self,
        f: impl FnOnce() + Send + 'static,
    ) -> Result<TerminationHandle, Self::ExecError>;
}

/// An extension of thread pools specifically for ones that use web workers.
/// Passing in the shim url ensures that we don't create multiple copies of it.
pub trait WorkerPool: ThreadPool {
    fn from_url(
        initial: usize,
        termination_strategy: TerminationStrategy,
        wasm_bindgen_shim_url: &str,
    ) -> Result<Self, Self::NewError>
    where
        Self: Sized;
}

/// As long as at least one clone of this handle exists,
/// the termination flag for a worker is set to false,
/// as it means that the result is still required.
#[derive(Clone)]
pub struct TerminationHandle {
    inner: Arc<InnerHandle>,
}

impl TerminationHandle {
    pub fn new(result_needed: Arc<AtomicBool>) -> Self {
        Self {
            inner: Arc::new(InnerHandle { result_needed }),
        }
    }
    pub fn cancel(&self) {
        self.inner.cancel()
    }
}

/// A handle which sets the termination flag for the associated worker.
/// This will allow the thread pool to terminate workers whose results are no longer required.
pub struct InnerHandle {
    result_needed: Arc<AtomicBool>,
}

impl InnerHandle {
    pub fn cancel(&self) {
        self.result_needed.store(false, Ordering::SeqCst)
    }
}

impl Drop for InnerHandle {
    fn drop(&mut self) {
        self.cancel()
    }
}
