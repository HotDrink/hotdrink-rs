#![feature(test)]
#![feature(result_flattening)]
#![feature(stmt_expr_attributes)]
#![feature(drain_filter)]
#![feature(concat_idents)]

pub mod algorithms;
pub mod data;
pub mod event;
pub mod examples;
pub mod macros;
pub mod thread;
pub mod variable_ranking;

pub use data::{
    traits::{ComponentLike, ConstraintLike, MethodLike},
    Component, Constraint, ConstraintSystem, Method,
};
