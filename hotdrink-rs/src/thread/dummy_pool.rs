//! A thread pool with no actual additional threads.
//! It will execute the work on the main thread.
//! Commonly used for testing and benchmarking.

use super::thread_pool::{TerminationHandle, ThreadPool};

/// A thread pool with no actual additional threads.
/// It will execute the work on the main thread.
/// Commonly used for testing and benchmarking.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct DummyPool;

impl ThreadPool for DummyPool {
    type NewError = bool;
    type ExecError = bool;
    fn new(_: usize) -> Result<Self, Self::NewError> {
        Ok(DummyPool)
    }

    fn execute(
        &mut self,
        f: impl FnOnce() + Send + 'static,
    ) -> Result<TerminationHandle, Self::ExecError> {
        f();
        let (th, _) = TerminationHandle::new();
        Ok(th)
    }
}
