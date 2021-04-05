//! Extra information about a variable, such as its status, generation, and callbacks.

use super::{generation_id::GenerationId, variable_activation::EventCallback};
use crate::event::{Event, GeneralEvent};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

/// Information about a variable.
///
/// More specifically, this struct contains its generation, current status, and a callback if one exists.
#[derive(Clone)]
pub struct FilteredCallback<T, E> {
    target: GenerationId,
    callback: Option<EventCallback<T, E>>,
}

impl<T, E> Default for FilteredCallback<T, E> {
    fn default() -> Self {
        Self {
            target: Default::default(),
            callback: None,
        }
    }
}

impl<T: Clone, E: Clone> FilteredCallback<T, E> {
    /// Constructs a new [`VariableInfo`] with the specified status.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns a reference to the callback of the variable.
    pub fn callback(&self) -> &Option<EventCallback<T, E>> {
        &self.callback
    }
    /// Sets the callback of the variable.
    pub fn subscribe(&mut self, callback: impl Fn(Event<T, E>) + Send + 'static) {
        self.callback = Some(Arc::new(Mutex::new(callback)));
    }
    /// Removes the callback of the variable.
    pub fn unsubscribe(&mut self) {
        self.callback = None;
    }

    /// Set the kind of events to respond to.
    pub fn set_target(&mut self, target: GenerationId) {
        self.target = target;
    }

    /// Calls the callback of the variable if one exists.
    ///
    /// Old events will be ignored, and new ones will update the current status of the variable.
    pub fn call_callback(&self, ge: GeneralEvent<T, E>) {
        let generation = ge.generation();

        // Ignore events from another generation
        if generation != self.target {
            return;
        }

        // Call callback
        if let Some(callback) = &self.callback {
            callback.lock().unwrap()(ge.event());
        }
    }
}

impl<T: Debug, E: Debug> Debug for FilteredCallback<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FilteredCallback")
            .field("target", &self.target)
            .finish()
    }
}

impl<T, E> PartialEq for FilteredCallback<T, E> {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
    }
}

impl<T, E> Eq for FilteredCallback<T, E> {}
