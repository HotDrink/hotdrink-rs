//! A wrapper around events that contains information about which variable was updated.

use hotdrink_rs::event::Event;

/// A wrapper around events that contains information about which variable was updated.
#[derive(Debug)]
pub struct JsEvent<T, E> {
    component: String,
    variable: String,
    event: Event<T, E>,
}

impl<T, E> JsEvent<T, E> {
    /// Constructs a new `JsEvent` with variable information and an event.
    pub fn new(component: String, variable: String, event: Event<T, E>) -> Self {
        Self {
            component,
            variable,
            event,
        }
    }

    /// Returns the name of the component the event is from.
    pub fn get_component(&self) -> &str {
        &self.component
    }

    /// Returns the name of the variable the event is about.
    pub fn get_variable(&self) -> &str {
        &self.variable
    }

    /// Returns the event that happened.
    pub fn get_event(self) -> Event<T, E> {
        self.event
    }
}
