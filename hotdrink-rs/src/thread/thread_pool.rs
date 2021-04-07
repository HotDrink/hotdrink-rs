//! Traits and types for thread pools with cancellation-capabilities.

use std::fmt::Debug;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

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

/// A trait for thread pool implementations.
pub trait ThreadPool {
    /// An error for when a new thread pool could not be constructed.
    type NewError: Debug;
    /// An error for when executing a task on the thread pool fails.
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

/// As long as at least one clone of this handle exists,
/// the termination flag for a worker is set to false.
#[derive(Clone, Debug, Default)]
pub struct TerminationHandle {
    inner: Arc<InnerHandle>,
}

impl TerminationHandle {
    /// Constructs a new termination handle.
    pub fn new() -> (Self, Arc<AtomicBool>) {
        let result_needed = Arc::new(AtomicBool::new(true));
        let inner_handle = Self {
            inner: Arc::new(InnerHandle {
                result_needed: result_needed.clone(),
            }),
        };
        (inner_handle, result_needed)
    }
}

/// A handle which sets the termination flag for an associated worker.
/// This will allow the thread pool to terminate workers whose results are no longer required.
/// The flag will be set when all references to this handle are dropped, or
/// it is cancelled manually.
#[derive(Clone, Debug, Default)]
pub struct InnerHandle {
    result_needed: Arc<AtomicBool>,
}

impl InnerHandle {
    /// Sets a flag to indicate that the result of the associated computation is no longer needed.
    pub fn cancel(&self) {
        self.result_needed.store(false, Ordering::SeqCst)
    }
}

impl Drop for InnerHandle {
    fn drop(&mut self) {
        self.cancel()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables, clippy::mutex_atomic)]

    use super::TerminationHandle;
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
