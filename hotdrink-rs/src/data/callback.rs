use std::sync::Arc;

/// A callback that reacts to updates from a variable
#[derive(Clone)]
pub struct Callback<T, E> {
    /// The value is being computed
    on_pending: Option<Arc<dyn Fn() + Send>>,
    /// The value is ready
    on_ready: Option<Arc<dyn Fn(&T) + Send>>,
    /// An error occured during computation
    on_error: Option<Arc<dyn Fn(E) + Send>>,
}

impl<T, E> Default for Callback<T, E> {
    fn default() -> Self {
        Self {
            on_pending: None,
            on_ready: None,
            on_error: None,
        }
    }
}

impl<T, E> Callback<T, E> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_pending(&mut self, pending: Arc<dyn Fn() + Send>) {
        self.on_pending = Some(pending)
    }

    pub fn set_ready(&mut self, ready: Arc<dyn Fn(&T) + Send>) {
        self.on_ready = Some(ready)
    }

    pub fn set_error(&mut self, error: Arc<dyn Fn(E) + Send>) {
        self.on_error = Some(error)
    }

    pub fn call_pending(&self) -> Option<()> {
        self.on_pending.as_ref().map(|on_pending| on_pending())
    }

    pub fn call_ready(&self, value: &T) -> Option<()> {
        self.on_ready.as_ref().map(|on_ready| on_ready(value))
    }

    pub fn call_error(&self, error: E) -> Option<()> {
        self.on_error.as_ref().map(|on_error| on_error(error))
    }
}
