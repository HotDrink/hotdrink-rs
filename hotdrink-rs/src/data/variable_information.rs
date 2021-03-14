use super::variable_activation::EventCallback;
use crate::event::{Event, GeneralEvent};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Pending,
    Ready,
    Error,
}

#[derive(Clone)]
pub struct VariableInfo<T, E> {
    generation: usize,
    status: Status,
    callback: Option<EventCallback<T, E>>,
}

impl<T: Clone, E: Clone> VariableInfo<T, E> {
    pub fn new(status: Status) -> Self {
        Self {
            generation: 0,
            status,
            callback: None,
        }
    }
    pub fn generation(&self) -> usize {
        self.generation
    }
    pub fn set_generation(&mut self, generation: usize) {
        self.generation = generation;
    }
    pub fn callback(&self) -> &Option<EventCallback<T, E>> {
        &self.callback
    }
    pub fn subscribe(&mut self, callback: impl Fn(Event<T, E>) + Send + 'static) {
        self.callback = Some(Arc::new(Mutex::new(callback)));
    }
    pub fn unsubscribe(&mut self) {
        self.callback = None;
    }
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
