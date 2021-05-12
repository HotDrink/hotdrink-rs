//! Types and functions for scheduling methods in plans generated by a [`planner`](crate::planner)s.

#[allow(clippy::module_inception)]
mod scheduler;
mod solve_error;

pub(crate) use scheduler::schedule;
pub use solve_error::{Reason, SolveError};
