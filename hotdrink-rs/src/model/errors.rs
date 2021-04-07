//! Errors from the API of [`ConstraintSystem`](crate::model::ConstraintSystem) and [`Component`](crate::model::Component).

use std::fmt::Display;

use super::generations::{NoMoreRedo, NoMoreUndo};

/// An error occured while using the API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApiError<'a> {
    /// See [`NoSuchComponent`].
    NoSuchComponent(NoSuchComponent<'a>),
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
            ApiError::NoSuchComponent(nsc) => nsc.fmt(f),
            ApiError::NoSuchVariable(nsv) => nsv.fmt(f),
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

/// The specified variable does not exist.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NoSuchVariable<'a>(pub &'a str);

impl<'a> Display for NoSuchVariable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable not found: {}", self.0)
    }
}
