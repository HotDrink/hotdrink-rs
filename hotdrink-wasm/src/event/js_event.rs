//! A wrapper around events that contains information about which variable was updated.

use hotdrink_rs::Event;

/// An event from the constraint system.
#[derive(Debug)]
pub enum JsEventInner<T, E> {
    /// The value is being computed.
    Pending,
    /// The computation succeeded.
    Ready(T),
    /// The computation failed.
    Error(Vec<E>),
}

impl<'a, T, E> From<Event<'a, T, E>> for JsEventInner<T, E>
where
    T: Clone,
{
    fn from(e: Event<'a, T, E>) -> Self {
        match e {
            Event::Pending => JsEventInner::Pending,
            Event::Ready(value) => JsEventInner::Ready(value.clone()),
            Event::Error(errors) => JsEventInner::Error(errors),
        }
    }
}

/// A wrapper around events that contains information about which variable was updated.
#[derive(Debug)]
pub struct JsEvent<T, E> {
    component: String,
    variable: String,
    data: JsEventInner<T, E>,
}

impl<T, E> JsEvent<T, E> {
    /// Constructs a new `JsEvent` with variable information and an event.
    pub fn new(component: String, variable: String, data: JsEventInner<T, E>) -> Self {
        Self {
            component,
            variable,
            data,
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
    pub fn into_inner(self) -> JsEventInner<T, E> {
        self.data
    }
}
