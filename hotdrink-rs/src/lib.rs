//! HotDrink implemented in Rust.
//!
//! # Prerequisites
//!
//! Begin by downloading the prerequisites below.
//!
//! * Rust (nightly)
//!
//! To use Rust nightly for this project, either set it globally with `rustup default nightly`, or for this project with `rustup override set nightly`.
//! It is required for the experimental benchmarking features.
//!
//! # Build
//!
//! ```bash
//! cargo build
//! ```

#![warn(missing_debug_implementations, rust_2018_idioms)]
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
