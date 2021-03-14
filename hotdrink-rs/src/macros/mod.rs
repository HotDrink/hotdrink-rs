//! Macros and types for generating components.

pub mod component_macro;
pub mod dummy_component;
pub mod raw_component;
pub mod raw_constraint;
pub mod raw_method;
pub mod value_macro;

pub use raw_component::RawComponent;
pub use raw_constraint::RawConstraint;
pub use raw_method::RawMethod;
