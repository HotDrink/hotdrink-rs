//! HotDrink implemented in Rust.
//!
//! # Prerequisites
//!
//! The project uses multiple nightly features, and must be built using nightly Rust.
//! I recommend using `rustup`, which can be downloaded [here](https://rustup.rs/),
//!
//! # Build
//!
//! If an appropriate version of Rust is installed, it should be as simple as running the following:
//!
//! ```bash
//! cargo build --release
//! ```

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
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
    component::Component,
    constraint::Constraint,
    constraint_system::ConstraintSystem,
    method::Method,
    traits::{ComponentSpec, ConstraintSpec, MethodFailure, MethodSpec, MethodResult},
};
