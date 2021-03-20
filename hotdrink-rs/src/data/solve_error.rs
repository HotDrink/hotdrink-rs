//! Errors that can happen during solving of a constraint system.

use super::traits::MethodFailure;
use crate::builders::method_builder::MutabilityMismatch;
use std::fmt::Display;

/// Information about an error that occured during solving.
#[derive(Clone, Debug, PartialEq)]
pub struct SolveError {
    component: String,
    constraint: String,
    method: String,
    reason: Reason,
}

impl SolveError {
    /// Constructs a new [`SolveError`] with location information (`component`, `constraint`, and `method`),
    /// as well as a [`Reason`] describing what went wrong.
    pub fn new(component: String, constraint: String, method: String, reason: Reason) -> Self {
        Self {
            component,
            constraint,
            method,
            reason,
        }
    }
}

impl Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match &self.reason {
            Reason::PreConditionFailure(name) => format!("Precondition {} did not hold.", name),
            Reason::PostConditionFailure(name) => format!("Postcondition {} did not hold.", name),
            Reason::MethodFailure(mf) => match mf {
                MethodFailure::NoSuchVariable(name) => format!("Unknown variable {}.", name),
                MethodFailure::TypeConversionFailure(name, ty) => {
                    format!("Variable {} could not be converted to {}.", name, ty)
                }
                MethodFailure::WrongInputCount(expected, actual) => {
                    format!("Method takes {} input(s), but got {}.", expected, actual)
                }
                MethodFailure::WrongOutputCount(expected, actual) => {
                    format!("Method takes {} output(s), but got {}.", expected, actual)
                }
                MethodFailure::Custom(msg) => msg.to_string(),
                MethodFailure::MutabilityMismatch(mm) => match mm {
                    MutabilityMismatch::ExpectedImmutableGotMutable => {
                        "Expected immutable reference, got mutable".to_string()
                    }
                    MutabilityMismatch::ExpectedMutableGotImmutable => {
                        "Expected mutable reference, got immutable".to_string()
                    }
                },
            },
        };
        write!(
            f,
            "{}.{}.{}: {}",
            self.component, self.constraint, self.method, message
        )
    }
}

/// A description of what went wrong during solving.
#[derive(Clone, Debug, PartialEq)]
pub enum Reason {
    /// A precondition did not hold before enforcing a constraint.
    PreConditionFailure(String),
    /// A postcondition did not hold after enforcing a constraint.
    PostConditionFailure(String),
    /// A method failed to execute. See [`MethodFailure`].
    MethodFailure(MethodFailure),
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::PreConditionFailure(msg) => write!(f, "a precondition not holding: {}", msg),
            Reason::PostConditionFailure(msg) => write!(f, "a postcondition not holding: {}", msg),
            Reason::MethodFailure(me) => write!(f, "a method failure: {:?}", me),
        }
    }
}
