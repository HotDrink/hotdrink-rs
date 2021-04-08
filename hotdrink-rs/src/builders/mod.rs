//! Builders for creating components manually, a new variant of the component macro,
//! and value-experiments for allowing mutation of values in the constraint system.

#![allow(unused_macros)]

#[macro_use]
mod sum_type;
#[macro_use]
mod dyn_fn;
#[macro_use]
mod method_macro;
#[macro_use]
mod constraint_macro;
#[macro_use]
mod component_macro;

pub mod component_builder;
pub mod constraint_builder;
pub mod method_builder;
pub mod value_experiments;

pub use component_builder::ComponentBuilder;
pub use constraint_builder::ConstraintBuilder;
pub use method_builder::MethodArg;
pub use method_builder::MethodBuilder;
pub use method_builder::MethodInput;
pub use method_builder::MethodOutput;
pub use value_experiments::Value;
