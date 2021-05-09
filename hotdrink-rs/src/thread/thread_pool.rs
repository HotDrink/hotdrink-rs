//! A trait for threadpool-like types.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{fmt::Debug, sync::atomic::AtomicUsize};

/// A trait for thread pool implementations.
pub trait ThreadPool {
    /// An error for when a new thread pool could not be constructed.
    type NewError: Debug;
    /// An error for when executing a task on the thread pool fails.
    type ExecError: Debug;

    /// Creates a new thread pool with the specified number of initial workers.
    fn new(initial: usize) -> Result<Self, Self::NewError>
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
#[derive(Debug, Default)]
pub struct TerminationHandle {
    inner: Arc<InnerHandle>,
    num_references: Arc<AtomicUsize>,
}

impl Clone for TerminationHandle {
    fn clone(&self) -> Self {
        self.num_references.fetch_add(1, Ordering::SeqCst);
        Self {
            inner: self.inner.clone(),
            num_references: self.num_references.clone(),
        }
    }
}

impl Drop for TerminationHandle {
    fn drop(&mut self) {
        self.num_references.fetch_sub(1, Ordering::SeqCst);
    }
}

impl TerminationHandle {
    /// Constructs a new termination handle.
    pub fn new() -> (Self, Arc<AtomicBool>) {
        let result_needed = Arc::new(AtomicBool::new(true));
        let inner_handle = Self {
            inner: Arc::new(InnerHandle {
                result_needed: result_needed.clone(),
            }),
            num_references: Arc::new(AtomicUsize::new(1)),
        };
        (inner_handle, result_needed)
    }

    /// Returns the number of references to this [`TerminationHandle`].
    pub fn num_references(&self) -> usize {
        self.num_references.load(Ordering::SeqCst)
    }
}

/// A handle which sets the termination flag for an associated worker.
/// This will allow the thread pool to terminate workers whose results are no longer required.
/// The flag will be set when all references to this handle are dropped, or
/// it is cancelled manually.
#[derive(Clone, Debug, Default)]
struct InnerHandle {
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
