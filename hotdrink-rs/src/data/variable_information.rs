//! Extra information about a variable, such as its status, generation, and callbacks.

use super::variable_activation::EventCallback;
use crate::event::{Event, GeneralEvent};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

/// The current status of a variable.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// A new value is being computed.
    Pending,
    /// The variable's value is fully updated.
    Ready,
    /// The variable is in an error-state.
    Error,
}

/// Information about a variable.
///
/// More specifically, this struct contains its generation, current status, and a callback if one exists.
#[derive(Clone)]
pub struct VariableInfo<T, E> {
    generation: usize,
    status: Status,
    callback: Option<EventCallback<T, E>>,
}

impl<T: Clone, E: Clone> VariableInfo<T, E> {
    /// Constructs a new [`VariableInfo`] with the specified status.
    pub fn new(status: Status) -> Self {
        Self {
            generation: 0,
            status,
            callback: None,
        }
    }
    /// Returns the current generation of the variable.
    pub fn generation(&self) -> usize {
        self.generation
    }
    /// Sets the generation to the specified value.
    pub fn set_generation(&mut self, generation: usize) {
        self.generation = generation;
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
    /// Calls the callback of the variable if one exists.
    ///
    /// Old events will be ignored, and new ones will update the current status of the variable.
    pub fn call_callback(&mut self, ge: GeneralEvent<T, E>) {
        let new_generation = ge.generation();
        let old_generation = self.generation();

        // Ignore old events
        if new_generation < old_generation {
            return;
        }

        // Update generation
        self.generation = new_generation;
        // Update status
        let event = ge.event();
        self.status = match &event {
            Event::Pending => Status::Pending,
            Event::Ready(_) => Status::Ready,
            Event::Error(_) => Status::Error,
        };
        // Call callback
        if let Some(callback) = &self.callback {
            callback.lock().unwrap()(event);
        }
    }
}

impl<T: Debug, E: Debug> Debug for VariableInfo<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariableInfo")
            .field("generation", &self.generation)
            .field("status", &self.status)
            .finish()
    }
}

impl<T, E> PartialEq for VariableInfo<T, E> {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation && self.status == other.status
    }
}

impl<T, E> Eq for VariableInfo<T, E> {}
