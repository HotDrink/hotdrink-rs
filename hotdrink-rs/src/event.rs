//! Types for representing events from the constraint system.

use crate::model::generation_id::GenerationId;
use std::fmt::Debug;

/// An event from the constraint system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event<'a, T, E> {
    /// The value is being computed.
    Pending,
    /// The computation succeeded.
    Ready(&'a T),
    /// The computation failed.
    Error(&'a Vec<E>),
}

/// An event from [`ConstraintSystem::update`](crate::model::ConstraintSystem::update) with information about
/// which variable it is, and which generation the computation is from.
#[derive(Debug)]
pub(crate) struct EventWithLocation<'a, T, E> {
    variable: usize,
    generation: GenerationId,
    event: Event<'a, T, E>,
}

impl<'a, T, E> EventWithLocation<'a, T, E> {
    /// Constructs a new [`EventWithLocation`] for the specified variable.
    ///
    /// This includes the generation the computation is from, and what the event is.
    pub fn new(variable: usize, generation: GenerationId, event: Event<'a, T, E>) -> Self {
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
    pub fn event(self) -> Event<'a, T, E> {
        self.event
    }
}
