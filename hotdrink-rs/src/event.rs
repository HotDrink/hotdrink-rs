//! Types for representing events from the constraint system.

use crate::model::generation_id::GenerationId;
use std::{fmt::Debug, sync::Arc};

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
pub enum SolveEvent<T, E> {
    /// The value is being computed.
    Pending,
    /// The computation succeeded.
    Ready(Arc<T>),
    /// The computation failed.
    Error(Vec<E>),
}

/// An attempt to avoid [`Arc`] in callbacks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event<'a, T, E> {
    /// The value is being computed.
    Pending,
    /// The computation succeeded.
    Ready(&'a T),
    /// The computation failed.
    Error(Vec<E>),
}

impl<'a, T, E> From<Event<'a, T, E>> for SolveEvent<T, E>
where
    T: Clone,
{
    fn from(e: Event<'a, T, E>) -> Self {
        match e {
            Event::Pending => SolveEvent::Pending,
            Event::Ready(value) => SolveEvent::Ready(Arc::new((*value).clone())),
            Event::Error(errors) => SolveEvent::Error(errors),
        }
    }
}

/// An event from [`ConstraintSystem::update`](crate::ConstraintSystem::update) with information about
/// which variable it is, and which generation the computation is from.
#[derive(Debug)]
pub(crate) struct SolveEventWithLoc<T, E> {
    variable: usize,
    generation: GenerationId,
    event: SolveEvent<T, E>,
}

impl<T, E> SolveEventWithLoc<T, E> {
    /// Constructs a new [`GeneralEvent`] for the specified variable.
    ///
    /// This includes the generation the computation is from, and what the event is.
    pub fn new(variable: usize, generation: GenerationId, event: SolveEvent<T, E>) -> Self {
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
    pub fn generation(&self) -> GenerationId {
        self.generation
    }

    /// Returns the actual event.
    pub fn event(self) -> SolveEvent<T, E> {
        self.event
    }
}
