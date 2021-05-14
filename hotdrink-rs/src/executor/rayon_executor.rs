use super::{MethodExecutor, TerminationHandle};

impl MethodExecutor for rayon::ThreadPool {
    type ExecError = ();
    fn schedule(
        &self,
        f: impl FnOnce() + Send + 'static,
    ) -> Result<TerminationHandle, Self::ExecError> {
        let (th, _) = TerminationHandle::new();
        self.spawn(f);
        Ok(th)
    }
}

impl MethodExecutor for rayon::Scope<'_> {
    type ExecError = ();
    fn schedule(
        &self,
        f: impl FnOnce() + Send + 'static,
    ) -> Result<TerminationHandle, Self::ExecError> {
        let (th, _) = TerminationHandle::new();
        self.spawn(|_| f());
        Ok(th)
    }
}
