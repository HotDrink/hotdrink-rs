//! A trait and data structure for maintaining a ranking of variables based on when they were updated.
//! Variables that are updated later should have a higher priority.

/// A trait for defining a variable ranker.
pub trait VariableRanker {
    /// Creates a ranker with `size` variables.
    fn of_size(size: usize) -> Self;
    /// Gets the number of variables in the ranker.
    fn size(&self) -> usize;
    /// Tells the ranker that a variable has been updated.
    fn touch(&mut self, index: usize);
    /// Creates a ranking based on previous inputs.
    fn ranking(&self) -> Vec<usize>;
}
