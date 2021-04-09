//! A type for representing variables.

use super::undo::{NoMoreRedo, NoMoreUndo};
use std::{collections::VecDeque, ops::Deref};

/// A variable that maintains its previous values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable<T> {
    index: usize,
    activations: VecDeque<T>,
}

impl<T: Default> Default for Variable<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<T> for Variable<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Deref for Variable<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> Variable<T> {
    /// Constructs a new [`Variable`] with the specified value.
    pub fn new(value: T) -> Self {
        let mut activations = VecDeque::with_capacity(1);
        activations.push_back(value);
        Self {
            index: 0,
            activations,
        }
    }
    /// Gives the variable a new value.
    pub fn set(&mut self, value: T) {
        self.activations.truncate(self.index + 1);
        self.activations.push_back(value);
        self.index += 1;
    }
    /// Returns the current value of the variable.
    pub fn get(&self) -> &T {
        &self.activations[self.index]
    }
    /// Switches to the previous value of the variable.
    pub fn undo(&mut self) -> Result<(), NoMoreUndo> {
        if self.index > 0 {
            self.index -= 1;
            Ok(())
        } else {
            Err(NoMoreUndo)
        }
    }
    /// Switches to the next value of the variable.
    pub fn redo(&mut self) -> Result<(), NoMoreRedo> {
        if self.index < self.activations.len() - 1 {
            self.index += 1;
            Ok(())
        } else {
            Err(NoMoreRedo)
        }
    }
    /// Pops the first
    pub(crate) fn pop_front(&mut self) -> Option<T> {
        assert!(
            self.activations.len() > 1,
            "Must always have at least one value"
        );
        self.index -= 1;
        self.activations.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::Variable;
    use crate::model::undo::{NoMoreRedo, NoMoreUndo};

    #[test]
    fn variable_has_correct_value() {
        let mut variable = Variable::new(0);
        assert_eq!(variable.get(), &0);
        variable.set(1);
        assert_eq!(variable.get(), &1);
        variable.set(2);
        assert_eq!(variable.get(), &2);
    }

    #[test]
    fn undo_then_redo_works() {
        let mut variable = Variable::new(0);
        variable.set(3);
        assert_eq!(variable.get(), &3);
        assert_eq!(variable.undo(), Ok(()));
        assert_eq!(variable.undo(), Err(NoMoreUndo));
        assert_eq!(variable.get(), &0);
        assert_eq!(variable.redo(), Ok(()));
        assert_eq!(variable.redo(), Err(NoMoreRedo));
        assert_eq!(variable.get(), &3);
    }

    #[test]
    fn set_clears_redo() {
        let mut variable = Variable::new(0);
        variable.set(2);
        variable.undo().unwrap();
        variable.set(3);
        assert_eq!(variable.redo(), Err(NoMoreRedo));
    }
}
