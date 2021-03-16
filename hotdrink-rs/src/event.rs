//! Types for representing events from the constraint system.

use std::fmt::Debug;

/// Uniquely identifies a variable in a component
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Identifier {
    component: String,
    variable: String,
}

impl Identifier {
    /// Returns the name of the component part of this identifier.
    pub fn component(&self) -> &str {
        &self.component
    }
    /// Returns the name of the variable part of this identifier.
    pub fn variable(&self) -> &str {
        &self.variable
    }
}

impl Identifier {
    /// Constructs a new `Identifier`.
    pub fn new(component: &str, variable: &str) -> Self {
        Self {
            component: component.to_string(),
            variable: variable.to_string(),
        }
    }
}

/// An event from the constraint system.
#[derive(Debug)]
pub enum Event<T, E> {
    /// The value is being computed.
    Pending,
    /// The computation succeeded.
    Ready(T),
    /// The computation failed.
    Error(Vec<E>),
}

/// An event from [`ConstraintSystem::update`](crate::ConstraintSystem::update) with information about
/// which variable it is, and which generation the computation is from.
#[derive(Debug)]
pub struct GeneralEvent<T, E> {
    variable: usize,
    generation: usize,
    event: Event<T, E>,
}

impl<T, E> GeneralEvent<T, E> {
    /// Constructs a new [`GeneralEvent`] for the specified variable.
    ///
    /// This includes the generation the computation is from, and what the event is.
    pub fn new(variable: usize, generation: usize, event: Event<T, E>) -> Self {
        Self {
            variable,
            generation,
            event,
        }
    }

    /// Returns the variable the event is for.
    pub fn variable(&self) -> usize {
        self.variable
    }

    /// Returns the generation the event is from.
    pub fn generation(&self) -> usize {
        self.generation
    }

    /// Returns the actual event.
    pub fn event(self) -> Event<T, E> {
        self.event
    }
}
