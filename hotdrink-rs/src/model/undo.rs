//! Errors for undo and redo.

use std::fmt::Display;

/// The limit on how much undo history to keep.
#[derive(derivative::Derivative)]
#[derivative(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum UndoLimit {
    /// No limit on the undo history.
    #[derivative(Default)]
    Unlimited,
    /// A limit on the undo history.
    Limited(usize),
}

/// Nothing more to undo.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct NoMoreUndo;

impl Display for NoMoreUndo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nothing more to undo")
    }
}

/// Nothing more to redo.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct NoMoreRedo;

impl Display for NoMoreRedo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nothing more to redo")
    }
}

// Annoying to use, must import trait.
// pub trait Undo {
//     fn undo(&mut self) -> Result<(), NoMoreUndo>;
//     fn redo(&mut self) -> Result<(), NoMoreRedo>;
// }
