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
    pub fn new(component: &str, variable: &str) -> Self {
        Self {
            component: component.to_string(),
            variable: variable.to_string(),
        }
    }
}

/// An event from the constraint system.
/// A variable can be
/// 1. Pending, meaning it is being computed.
/// 2. Ready, meaning it was computed successfully.
/// 3. Error, meaning it was not computed successfully.
#[derive(Debug)]
pub enum Event<T, E> {
    Pending,
    Ready(T),
    Error(Vec<E>),
}

#[derive(Debug)]
pub struct GeneralEvent<T, E> {
    variable: usize,
    generation: usize,
    event: Event<T, E>,
}

impl<T, E> GeneralEvent<T, E> {
    pub fn new(variable: usize, generation: usize, event: Event<T, E>) -> Self {
        Self {
            variable,
            generation,
            event,
        }
    }

    pub fn variable(&self) -> usize {
        self.variable
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn event(self) -> Event<T, E> {
        self.event
    }
}
