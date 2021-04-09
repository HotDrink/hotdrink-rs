//! HotDrink implemented in Rust.
//!
//! HotDrink lets you describe relations between values declaratively and how to enforce them,
//! and can then automatically do so when the value of a variable changes.
//!
//! # Introduction
//!
//! Before getting started, here is a quick introduction to the terminology and how it works.
//! A [`Component`](crate::model::Component) is a set of variables with a set of [`Constraint`](crate::model::Constraint)s between them.
//! A `Constraint` consists of a set of [`Method`](crate::model::Method)s that are essentially functions that enforce the constraint
//! by reading from some subset of the variables of the `Component` and writing to another.
//! `Components` can be gathered in a [`ConstraintSystem`](crate::model::ConstraintSystem), which provides an API
//! for interacting with multiple `Component`s at once, such as [`update`](crate::model::ConstraintSystem::update).
//!
//! ## Components
//!
//! A *component* is a collection of variables and constraints between them that should be enforced.
//! One can easily be created by using the [`component!`] macro, as shown in the example below.
//!
//! ## Constraints
//!
//! A *constraint* represents a relation between variables we want to maintain.
//! It contains a collection of *constraint satisfaction methods* that describe the different ways to do so.
//! In the example, we want the relation `a + b = c` to hold at all times.
//! One way to enforce it is to re-compute `a + b` and set `c` to that value.
//!
//! ## Methods
//!
//! A *constraint satisfaction method* describes one way to enforce a constraint.
//! It reads the values of some variables, and write to others.
//!
//! # Examples
//!
//! ```
//! use hotdrink_rs::{component, model::ConstraintSystem, ret, Event};
//!
//! // Define a set of variables and relations between them
//! let mut component = component! {
//!     // Define a component `Component`.
//!     component Component {
//!         // Define variables and their default values.
//!         // The value can be omitted for any type that implements `Default`.
//!         let a: i32 = 0, b: i32, c: i32 = 3;
//!         // Define a constraint `Sum` that must hold between variables.
//!         constraint Sum {
//!             // Provide three ways to enforce the constraint.
//!             // Only one will be selected, so each one *MUST* enforce the constraint.
//!             abc(a: &i32, b: &i32) -> [c] = ret![*a + *b];
//!             acb(a: &i32, c: &i32) -> [b] = ret![*c - *a];
//!             bca(b: &i32, c: &i32) -> [a] = ret![*c - *b];
//!         }
//!     }
//! };
//!
//! // Describe what should happen when `a` changes.
//! component.subscribe("a", |event| match event {
//!     Event::Pending => println!("A new value for `a` is being computed"),
//!     Event::Ready(value) => println!("New value for `a`: {}", value),
//!     Event::Error(errors) => println!("Computation for `a` failed: {:?}", errors),
//! });
//!
//! // Change the value of `a`
//! component.set_variable("a", 3);
//!
//! // Enforce all the constraints by selecting a method for each one,
//! // and then executing the methods in topological order.
//! component.update();
//!
//! // Add the component to a constraint system.
//! // One constraint system can contain many components.
//! let mut cs = ConstraintSystem::new();
//! cs.add_component(component);
//!
//! // Update every component in the constraint system.
//! cs.update();
//! ```
//!
//! # Building
//!
//! The project uses multiple nightly features, and must be built using nightly Rust.
//! I recommend using `rustup`, which can be downloaded [here](https://rustup.rs/).
//!
//! If an appropriate version of Rust is installed, it should be as simple as running the following:
//!
//! ```bash
//! cargo build --release
//! ```

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    missing_docs
)]
#![feature(test)]
#![feature(result_flattening)]
#![feature(stmt_expr_attributes)]
#![feature(drain_filter)]
#![feature(concat_idents)]
#![feature(vec_into_raw_parts)]

#[macro_use]
pub mod macros;
pub mod builders;
pub(crate) mod event;
pub mod examples;
pub mod model;
pub mod planner;
pub mod solver;
pub mod thread;
pub mod util;
pub(crate) mod variable_ranking;

pub use event::Event;
