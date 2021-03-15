//! Data types used for representing constraint systems, with the most important ones being the following:
//!
//! 1. [`ConstraintSystem`](crate::ConstraintSystem)
//! 2. [`Component`](crate::Component)
//! 3. [`Constraint`](crate::Component)
//! 4. [`Method`](crate::Component)
//!
//! As well as other types used in the API.

pub mod component;
pub mod constraint;
pub mod constraint_system;
pub mod method;
pub mod solve_error;
pub mod traits;
pub mod variable_activation;
pub mod variable_information;
