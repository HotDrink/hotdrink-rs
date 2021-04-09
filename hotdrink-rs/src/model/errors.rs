//! Errors from the API of [`ConstraintSystem`](crate::model::ConstraintSystem) and [`Component`](crate::model::Component).

use super::undo::{NoMoreRedo, NoMoreUndo};
use std::fmt::Display;

/// An error occured while using the API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApiError<'a> {
    /// See [`NoSuchComponent`].
    NoSuchComponent(NoSuchComponent<'a>),
    /// See [`NoSuchConstraint`].
    NoSuchConstraint(NoSuchConstraint<'a>),
    /// See [`NoSuchVariable`].
    NoSuchVariable(NoSuchVariable<'a>),
    /// Nothing more to undo.
    NoMoreUndo,
    /// Nothing more to redo.
    NoMoreRedo,
}

impl<'a> Display for ApiError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NoSuchComponent(e) => e.fmt(f),
            ApiError::NoSuchConstraint(e) => e.fmt(f),
            ApiError::NoSuchVariable(e) => e.fmt(f),
            ApiError::NoMoreUndo => write!(f, "Nothing more to undo"),
            ApiError::NoMoreRedo => write!(f, "Nothing more to redo"),
        }
    }
}

impl<'a> From<NoSuchComponent<'a>> for ApiError<'a> {
    fn from(nsc: NoSuchComponent<'a>) -> Self {
        Self::NoSuchComponent(nsc)
    }
}

impl<'a> From<NoSuchConstraint<'a>> for ApiError<'a> {
    fn from(nsc: NoSuchConstraint<'a>) -> Self {
        Self::NoSuchConstraint(nsc)
    }
}

impl<'a> From<NoSuchVariable<'a>> for ApiError<'a> {
    fn from(nsv: NoSuchVariable<'a>) -> Self {
        Self::NoSuchVariable(nsv)
    }
}

impl<'a> From<NoMoreUndo> for ApiError<'a> {
    fn from(_: NoMoreUndo) -> Self {
        Self::NoMoreUndo
    }
}

impl<'a> From<NoMoreRedo> for ApiError<'a> {
    fn from(_: NoMoreRedo) -> Self {
        Self::NoMoreRedo
    }
}

/// The specified component does not exist.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NoSuchComponent<'a>(pub &'a str);

impl<'a> Display for NoSuchComponent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component not found: {}", self.0)
    }
}

/// The specified constraint does not exist.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NoSuchConstraint<'a>(pub &'a str);

impl<'a> Display for NoSuchConstraint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Constraint not found: {}", self.0)
    }
}

/// The specified variable does not exist.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NoSuchVariable<'a>(pub &'a str);

impl<'a> Display for NoSuchVariable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable not found: {}", self.0)
    }
}
