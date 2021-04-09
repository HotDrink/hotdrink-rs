//! Extra information about a variable, such as its status, generation, and callbacks.

use super::{generation_id::GenerationId, activation::EventCallback};
use crate::event::{Event, EventWithLocation};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

/// Information about a variable.
///
/// More specifically, this struct contains its generation, current status, and a callback if one exists.
pub(crate) struct FilteredCallback<T, E> {
    target: GenerationId,
    callback: Option<EventCallback<T, E>>,
}

impl<T, E> Clone for FilteredCallback<T, E> {
    fn clone(&self) -> Self {
        Self {
            target: self.target,
            callback: self.callback.clone(),
        }
    }
}

impl<T, E> Default for FilteredCallback<T, E> {
    fn default() -> Self {
        Self {
            target: Default::default(),
            callback: None,
        }
    }
}

impl<T, E: Clone> FilteredCallback<T, E> {
    /// Constructs a new [`VariableInfo`] with the specified status.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the callback of the variable.
    pub fn subscribe(&mut self, callback: impl Fn(Event<'_, T, E>) + Send + 'static) {
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
    pub fn call(&self, ge: EventWithLocation<'_, T, E>) {
        let generation = ge.generation();

        // Ignore events from another generation
        if generation != self.target {
            return;
        }

        // Call callback
        if let Some(callback) = &self.callback {
            match ge.event() {
                Event::Pending => callback.lock().unwrap()(Event::Pending),
                Event::Ready(value) => callback.lock().unwrap()(Event::Ready(value)),
                Event::Error(errors) => callback.lock().unwrap()(Event::Error(errors)),
            };
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
