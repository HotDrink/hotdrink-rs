//! Data types used for representing constraint systems, with the most important ones being the following:
//!
//! 1. [`ConstraintSystem`](self::ConstraintSystem)
//! 2. [`Component`](self::Component)
//! 3. [`Constraint`](self::Component)
//! 4. [`Method`](self::Component)
//!
//! As well as other types used in the API.

mod component;
mod constraint;
mod constraint_system;
pub(crate) mod errors;
pub(crate) mod filtered_callback;
pub(crate) mod generation_id;
mod method;
pub(crate) mod solve_error;
mod spec;
pub(crate) mod undo_vec;
pub(crate) mod variable_activation;

pub use component::Component;
pub use constraint::Constraint;
pub use constraint_system::ConstraintSystem;
pub use method::Method;
pub use solve_error::{Reason, SolveError};
pub use spec::{
    ComponentSpec, ConstraintSpec, MethodFailure, MethodFunction, MethodResult, MethodSpec,
    PlanError,
};
pub use variable_activation::{DoneState, State};
