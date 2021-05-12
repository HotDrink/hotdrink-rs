//! A trait for threadpool-like types with cancellation-capabilities.

use hotdrink_rs::executor::{DummyExecutor, MethodExecutor};

/// Strategies for when to terminate workers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

/// An extension of thread pools specifically for ones that use web workers.
/// Passing in the shim url ensures that we don't create multiple copies of it.
pub trait WorkerPool: MethodExecutor {
    /// Constructs a new pool as usual, but with a specified
    /// path to the Web Worker source.
    /// This is useful to avoid creating many instances of the blob.
    fn from_url(
        initial: usize,
        termination_strategy: TerminationStrategy,
        wasm_bindgen_shim_url: &str,
    ) -> Result<Self, Self::NewError>
    where
        Self: Sized;
}

impl WorkerPool for DummyExecutor {
    fn from_url(initial: usize, _: TerminationStrategy, _: &str) -> Result<Self, Self::NewError>
    where
        Self: Sized,
    {
        Self::new(initial)
    }
}
