//! Errors from the API of [`ConstraintSystem`](crate::model::ConstraintSystem) and [`Component`](crate::model::Component).

use super::undo::{NoMoreRedo, NoMoreUndo};
use std::fmt::Display;

/// An error occured while using the API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NoSuchItem<'a> {
    /// See [`NoSuchComponent`].
    Component(NoSuchComponent<'a>),
    /// See [`NoSuchConstraint`].
    Constraint(NoSuchConstraint<'a>),
    /// See [`NoSuchVariable`].
    Variable(NoSuchVariable<'a>),
    /// Nothing more to undo.
    Undo,
    /// Nothing more to redo.
    Redo,
}

impl<'a> Display for NoSuchItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoSuchItem::Component(e) => e.fmt(f),
            NoSuchItem::Constraint(e) => e.fmt(f),
            NoSuchItem::Variable(e) => e.fmt(f),
            NoSuchItem::Undo => write!(f, "Nothing more to undo"),
            NoSuchItem::Redo => write!(f, "Nothing more to redo"),
        }
    }
}

impl<'a> From<NoSuchComponent<'a>> for NoSuchItem<'a> {
    fn from(nsc: NoSuchComponent<'a>) -> Self {
        Self::Component(nsc)
    }
}

impl<'a> From<NoSuchConstraint<'a>> for NoSuchItem<'a> {
    fn from(nsc: NoSuchConstraint<'a>) -> Self {
        Self::Constraint(nsc)
    }
}

impl<'a> From<NoSuchVariable<'a>> for NoSuchItem<'a> {
    fn from(nsv: NoSuchVariable<'a>) -> Self {
        Self::Variable(nsv)
    }
}

impl<'a> From<NoMoreUndo> for NoSuchItem<'a> {
    fn from(_: NoMoreUndo) -> Self {
        Self::Undo
    }
}

impl<'a> From<NoMoreRedo> for NoSuchItem<'a> {
    fn from(_: NoMoreRedo) -> Self {
        Self::Redo
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
