use hotdrink_rs::thread::{DummyPool, TerminationStrategy, ThreadPool};

/// An extension of thread pools specifically for ones that use web workers.
/// Passing in the shim url ensures that we don't create multiple copies of it.
pub trait WorkerPool: ThreadPool {
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

impl WorkerPool for DummyPool {
    fn from_url(
        initial: usize,
        termination_strategy: TerminationStrategy,
        _: &str,
    ) -> Result<Self, Self::NewError>
    where
        Self: Sized,
    {
        Self::new(initial, termination_strategy)
    }
}
