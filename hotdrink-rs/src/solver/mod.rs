//! Types and functions for using a plan to solve a constraint system.

mod solve_error;
#[allow(clippy::module_inception)]
mod solver;

pub use solve_error::{Reason, SolveError};
pub(crate) use solver::solve;
