//! A [`Vec`]-like data structure to allow undo and redo of operations.

use std::{collections::VecDeque, fmt::Display, ops::Index};

/// The limit on how much undo history to keep.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UndoLimit {
    /// No limit on the undo history.
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

/// Represents values over time to allow for undo and redo.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UndoVec<T> {
    /// The generation we are currently on.
    current_generation: usize,
    /// The index of the current value of each variable.
    current_idx: Vec<usize>,
    /// If values have been modified since last commit.
    is_modified: bool,
    /// A list of values for each variable.
    values: Vec<VecDeque<T>>,
    /// `diff[n]` gives the difference between generation `n` and `n+1`.
    diff: VecDeque<Vec<usize>>,
    /// The maximum number of generations to keep.
    undo_limit: UndoLimit,
}

impl<T> UndoVec<T> {
    /// Constructs a new [`UndoVec`] with the specified default values.
    pub fn new(start_values: Vec<T>) -> Self {
        Self {
            current_generation: 0,
            current_idx: vec![0; start_values.len()],
            values: start_values
                .into_iter()
                .map(|value| VecDeque::from(vec![value]))
                .collect(),
            is_modified: false,
            diff: Default::default(),
            undo_limit: UndoLimit::Unlimited,
        }
    }

    /// Constructs a new [`UndoVec`] with the specified default values and history limit.
    pub fn new_with_limit(start_values: Vec<T>, undo_limit: usize) -> Self {
        let mut without_cap = Self::new(start_values);
        without_cap.undo_limit = UndoLimit::Limited(undo_limit);
        without_cap
    }

    /// Sets a new limit on the number of undos to keep and enforce it.
    pub fn set_limit(&mut self, undo_limit: UndoLimit) {
        self.undo_limit = undo_limit;
        self.clear_past();
    }

    /// Returns the number of variables per generation.
    pub fn n_variables(&self) -> usize {
        self.values.len()
    }

    /// Returns the number of generations stored.
    pub fn generations(&self) -> usize {
        self.diff.len() + 1
    }

    /// Returns a reference to a specified variable.
    pub fn get(&self, index: usize) -> Option<&T> {
        let current_idx = *self.current_idx.get(index)?;
        let res = self.values.get(index)?.get(current_idx)?;
        Some(res)
    }

    /// Returns a mutable reference to a specified variable.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let current_idx = *self.current_idx.get(index)?;
        let res = self.values.get_mut(index)?.get_mut(current_idx)?;
        Some(res)
    }

    /// Clears the future.
    /// This function can be used to clear an future invalidated by modifying the past.
    fn clear_future(&mut self) {
        self.diff.truncate(self.current_generation);
        for (vi, current_index) in self.current_idx.iter().enumerate() {
            self.values[vi].truncate(current_index + 1);
        }
    }

    /// Begins a new generation: This includes incrementing the generation counter,
    /// and adding a diff from the previous generation.
    fn begin_generation(&mut self) {
        self.current_generation += 1;
        self.diff.push_back(Vec::new());
    }

    /// Deletes undo history that goes past the limit, if a limit exists.
    fn clear_past(&mut self) {
        // Delete old history that goes past the undo limit
        if let UndoLimit::Limited(undo_limit) = self.undo_limit {
            // While we have too many generations
            while self.generations() - 1 > undo_limit {
                // Pop the earliest diff
                let earliest_diff = self
                    .diff
                    .pop_front()
                    .expect("Diff did not have enough generations");
                // Pop earliest value for each variable
                for vi in earliest_diff {
                    self.values[vi].pop_front();
                    self.current_idx[vi] -= 1;
                }
                self.current_generation -= 1;
            }
        }
    }

    /// Gives a variable a new value.
    pub fn set(&mut self, index: usize, value: T) {
        self.clear_future();

        if !self.is_modified {
            self.begin_generation();
        }

        self.diff[self.current_generation - 1].push(index);
        self.current_idx[index] += 1;
        self.values[index].push_back(value);

        if !self.is_modified {
            self.clear_past();
        }

        self.is_modified = true;
    }

    /// Returns references to the current values.
    pub fn values(&self) -> Vec<&T> {
        self.values
            .iter()
            .zip(&self.current_idx)
            .map(|(v, &index)| &v[index])
            .collect()
    }

    /// Stores a checkpoint that can be returned to with [`undo`](#method.undo) or [`redo`](#method.redo).
    pub fn commit(&mut self) {
        self.is_modified = false;
    }

    /// Moves back to the last [`commit`](#method.commit).
    pub fn undo(&mut self) -> Result<(), NoMoreUndo> {
        if self.current_generation == 0 {
            return Err(NoMoreUndo);
        }

        self.current_generation -= 1;
        self.is_modified = false;

        // Move pointers for modified variables back one generation
        for &vid in &self.diff[self.current_generation] {
            self.current_idx[vid] -= 1;
        }

        Ok(())
    }

    /// Moves forward to the next [`commit`](#method.commit).
    pub fn redo(&mut self) -> Result<(), NoMoreRedo> {
        if self.current_generation == self.generations() - 1 {
            return Err(NoMoreRedo);
        }

        // Move pointers for modified variables forward one generation
        for &vid in &self.diff[self.current_generation] {
            self.current_idx[vid] += 1;
        }

        self.current_generation += 1;
        self.is_modified = false;

        Ok(())
    }
}

impl<T> Index<usize> for UndoVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> From<Vec<T>> for UndoVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::new(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::{NoMoreRedo, NoMoreUndo, UndoVec};

    #[test]
    fn new_has_correct_len() {
        let g = UndoVec::new(vec![1, 2, 3]);
        assert_eq!(g.n_variables(), 3);
    }

    #[test]
    fn new_has_correct_values() {
        let g = UndoVec::new(vec![1, 2, 3]);
        assert_eq!(g.values(), vec![&1, &2, &3])
    }

    #[test]
    fn has_correct_values_after_set() {
        let mut gs = UndoVec::new(vec![0]);
        gs.set(0, 3);
        assert_eq!(gs.values(), vec![&3]);
    }

    #[test]
    fn undo_at_start_is_idempotent() {
        let mut g = UndoVec::new(vec![0]);
        assert_eq!(g.undo(), Err(NoMoreUndo));
        assert_eq!(g.values(), vec![&0]);
    }

    #[test]
    fn do_then_undo_is_identity() {
        let mut gs = UndoVec::new(vec![0]);
        gs.set(0, 3);
        assert_eq!(gs.undo(), Ok(()));
        assert_eq!(gs.values(), vec![&0]);
    }

    #[test]
    fn redo_at_start_is_idempotent() {
        let mut g = UndoVec::new(vec![0]);
        assert_eq!(g.redo(), Err(NoMoreRedo));
        assert_eq!(g.values(), vec![&0]);
    }

    #[test]
    fn undo_redo_is_identity() {
        let mut gs = UndoVec::new(vec![0]);
        gs.set(0, 3);
        assert_eq!(gs.undo(), Ok(()));
        assert_eq!(gs.redo(), Ok(()));
        assert_eq!(gs.values(), vec![&3]);
    }

    #[test]
    fn set_deletes_redo_history() {
        let mut gs = UndoVec::new(vec![0]);
        gs.set(0, 3);
        gs.commit();
        gs.undo().unwrap();
        gs.set(0, 5);
        assert_eq!(gs.redo(), Err(NoMoreRedo));
        assert_eq!(gs.values(), vec![&5]);
    }

    #[test]
    fn do_undo_redo_mix() {
        let mut gs = UndoVec::new(vec![0, 0]);
        // Assert default values
        assert_eq!(gs.values(), vec![&0, &0]);
        // Try moving a bit, but only 1 generation
        assert_eq!(gs.undo(), Err(NoMoreUndo));
        assert_eq!(gs.redo(), Err(NoMoreRedo));
        // Set first values
        gs.set(0, 1);
        gs.set(1, 2);
        assert_eq!(gs.values(), vec![&1, &2]);
        // Undo the change
        assert_eq!(gs.undo(), Ok(()));
        assert_eq!(gs.values(), vec![&0, &0]);
        assert_eq!(gs.undo(), Err(NoMoreUndo));
        // Redo the change
        assert_eq!(gs.redo(), Ok(()));
        assert_eq!(gs.redo(), Err(NoMoreRedo));
        // Verify that change was redone
        assert_eq!(gs.values(), vec![&1, &2]);
        // Undo again
        assert_eq!(gs.undo(), Ok(()));
        gs.set(0, 5);
        gs.set(1, 6);
        // Make sure redo history was deleted
        assert_eq!(gs.redo(), Err(NoMoreRedo));
        assert_eq!(gs.values(), vec![&5, &6]);
    }

    #[test]
    fn undo_limit_zero_gives_no_undo() {
        // Without commit
        let mut gs = UndoVec::new_with_limit(vec![0], 0);
        gs.set(0, 3);
        assert_eq!(gs.undo(), Err(NoMoreUndo));

        // With commit
        let mut gs = UndoVec::new_with_limit(vec![0], 0);
        gs.set(0, 3);
        gs.commit();
        assert_eq!(gs.undo(), Err(NoMoreUndo));
    }

    #[test]
    fn undo_limit_one_gives_one_undo() {
        // Without commit
        let mut gs = UndoVec::new_with_limit(vec![0], 1);
        gs.set(0, 3);
        assert_eq!(gs.undo(), Ok(()));
        assert_eq!(gs.undo(), Err(NoMoreUndo));

        // With commit
        let mut gs = UndoVec::new_with_limit(vec![0], 1);
        gs.set(0, 3);
        gs.commit();
        assert_eq!(gs.undo(), Ok(()));
        assert_eq!(gs.undo(), Err(NoMoreUndo));
    }

    #[test]
    fn undo_limit_n_gives_n_undos() {
        for undo_limit in 0..10 {
            let mut gs = UndoVec::new_with_limit(vec![0], undo_limit);
            for _ in 0..undo_limit {
                gs.set(0, 1);
                gs.commit();
            }
            for _ in 0..undo_limit {
                assert_eq!(gs.undo(), Ok(()));
            }
            assert_eq!(gs.values(), vec![&0]);
            assert_eq!(gs.undo(), Err(NoMoreUndo));
        }
    }
}
