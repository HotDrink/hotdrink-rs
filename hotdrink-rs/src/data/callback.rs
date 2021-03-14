use std::fmt::Debug;
use std::sync::Arc;

/// A callback that reacts to updates from a variable.
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
    /// Constructs a new empty callback.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the pending callback.
    pub fn set_pending(&mut self, pending: Arc<dyn Fn() + Send>) {
        self.on_pending = Some(pending)
    }

    /// Sets the ready callback.
    pub fn set_ready(&mut self, ready: Arc<dyn Fn(&T) + Send>) {
        self.on_ready = Some(ready)
    }

    /// Sets the error callback.
    pub fn set_error(&mut self, error: Arc<dyn Fn(E) + Send>) {
        self.on_error = Some(error)
    }

    /// Calls the pending callback if one exists.
    pub fn call_pending(&self) -> Option<()> {
        self.on_pending.as_ref().map(|on_pending| on_pending())
    }

    /// Calls the ready callback if one exists.
    pub fn call_ready(&self, value: &T) -> Option<()> {
        self.on_ready.as_ref().map(|on_ready| on_ready(value))
    }

    /// Calls the error callback if one exists.
    pub fn call_error(&self, error: E) -> Option<()> {
        self.on_error.as_ref().map(|on_error| on_error(error))
    }
}

/// Creates a debug-str for any [`Option`]
/// without including its contents.
/// This is useful when it contains non-[`Debug`] types.
fn fmt_opt<T>(opt: &Option<T>) -> &'static str {
    match opt {
        Some(_) => "Some(..)",
        None => "None",
    }
}

impl<T, E> Debug for Callback<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Callback")
            .field("on_pending", &fmt_opt(&self.on_pending))
            .field("on_ready", &fmt_opt(&self.on_ready))
            .field("on_error", &fmt_opt(&self.on_error))
            .finish()
    }
}
