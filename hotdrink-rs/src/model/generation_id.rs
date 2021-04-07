//! An identifier used to know when a computation is from.

/// An identifier used to know when a computation is from.
///
/// It must have two components due to the following case:
/// Perform an action that moves from generation `n` to `n+1`,
/// then undo to `n`, then perform a new action that goes to `n+1` again.
/// The results from the first action can then be misinterpreted as values for the last action.
/// With `total_generation`, the first will have total `x`, and the next will have total `x+1`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct GenerationId {
    /// The generation the component was on.
    current_generation: usize,
    /// The total amount of generations the component has had.
    /// This can differ from `current_generation` because of undo.
    total_generation: usize,
}

impl GenerationId {
    /// Constructs a new [`GenerationId`] with `current_generation` and `total_generation` both set to 0.
    pub fn new(current_generation: usize, total_generation: usize) -> Self {
        Self {
            current_generation,
            total_generation,
        }
    }

    /// Returns the current generation.
    pub fn current_generation(&self) -> usize {
        self.current_generation
    }

    /// Returns the total generation.
    pub fn total_generation(&self) -> usize {
        self.current_generation
    }
}
