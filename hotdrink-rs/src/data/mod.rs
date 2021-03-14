//! Data types used for representing constraint systems, as well as operations on them like planning and solving.

pub mod callback;
pub mod component;
pub mod constraint;
pub mod constraint_system;
pub mod method;
pub mod solve_error;
pub mod traits;
pub mod variable_activation;
pub mod variable_information;

pub use component::Component;
pub use constraint::Constraint;
pub use constraint_system::ConstraintSystem;
pub use method::Method;
