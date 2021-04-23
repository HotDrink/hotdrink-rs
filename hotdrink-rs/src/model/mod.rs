//! Data types used for representing constraint systems, with the most important ones being the following:
//!
//! 1. [`ConstraintSystem`](self::ConstraintSystem)
//! 2. [`Component`](self::Component)
//! 3. [`Constraint`](self::Constraint)
//! 4. [`Method`](self::Method)
//!
//! As well as other types used in the API.

pub(crate) mod activation;
mod component;
mod constraint;
mod constraint_system;
pub(crate) mod errors;
pub(crate) mod filtered_callback;
pub(crate) mod generation_id;
mod method;
pub mod undo;
mod variable;
pub(crate) mod variables;

pub use activation::{Activation, Value};
pub use component::Component;
pub use constraint::Constraint;
pub use constraint_system::ConstraintSystem;
pub use method::Method;
pub use variable::Variable;
