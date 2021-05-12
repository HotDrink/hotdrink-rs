//! A single threaded method executor.
//! It will execute the work on the main thread.
//! Commonly used for testing and benchmarking.

use super::method_executor::{MethodExecutor, TerminationHandle};

/// A single threaded method executor.
/// It will execute the work on the main thread.
/// Commonly used for testing and benchmarking.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct DummyExecutor;

impl MethodExecutor for DummyExecutor {
    type NewError = bool;
    type ExecError = bool;
    fn new(_: usize) -> Result<Self, Self::NewError> {
        Ok(DummyExecutor)
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