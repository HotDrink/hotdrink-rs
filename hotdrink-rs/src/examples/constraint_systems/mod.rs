//! Utility functions for creating testable and benchmarkable constraint systems.
//! The different submodules allow construction of different kinds of constraint systems.

pub mod dense;
pub mod empty;
pub mod factory;
pub mod ladder;
pub mod linear;
pub mod sparse;
pub mod tree;

pub use dense::make_dense_cs;
pub use empty::make_empty_cs;
pub use linear::linear_oneway;
pub use sparse::make_sparse_cs;
pub use tree::multioutput_singleway;
