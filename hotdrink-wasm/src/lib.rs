//! A wrapper library around `hotdrink-rs` for compilation to WebAssembly.
//!
//! [![Crates.io][crates-badge]][crates-url]
//! [![docs.rs](https://docs.rs/hotdrink-wasm/badge.svg)](https://docs.rs/hotdrink-wasm)
//!
//! [crates-badge]: https://img.shields.io/crates/v/hotdrink-wasm.svg
//! [crates-url]: https://crates.io/crates/hotdrink-wasm
//!
//! # Prerequisites
//!
//! The project uses multiple nightly features, and must be built using nightly Rust.
//! I recommend using `rustup`, which can be downloaded [here](https://rustup.rs/),
//!
//! You also need `wasm-pack` to compile your project to WebAssembly, which can be downloaded [here](https://rustwasm.github.io/wasm-pack/installer/).
//!
//! The standard library must be recompiled with atomics enabled to use Web Workers as threads,
//! which means that we need the standard library source code.
//! This can be downloaded with `rustup component add rust-src`.
//!
//! See the parallel raytracing documentation <https://rustwasm.github.io/docs/wasm-bindgen/examples/raytrace.html> for more information.
//!
//! # Examples
//!
//! ## Single threaded
//!
//! ```rust
//! use hotdrink_rs::{component, model::ConstraintSystem};
//! use hotdrink_wasm::constraint_system_wrapper;
//! use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
//!
//! constraint_system_wrapper!(
//!     // A wrapper around `ConstraintSystem`.
//!     pub struct MyCs {
//!         // A wrapper that generates functions
//!         // for each variant of the inner value.
//!         pub struct ValueWrapper {
//!             // The actual value type inside the `ConstraintSystem`.
//!             #[derive(Clone, Debug)]
//!             pub enum Value {
//!                 i32,
//!                 String
//!             }
//!         }
//!     }
//! );
//!
//! // Use the wrapper.
//! #[wasm_bindgen]
//! pub fn make_cs() -> Result<MyCs, JsValue> {
//!     let mut cs = ConstraintSystem::new();
//!     cs.add_component(component! {
//!         component MyComponent {
//!             let a: i32 = 0, b: String = "";
//!             // <contraints>
//!         }
//!     });
//!     MyCs::wrap(cs)
//! }
//!
//! // Usage of `ValueWrapper`.
//! let cs = make_cs().unwrap();
//! cs.edit("MyComponent", "a", ValueWrapper::i32(5));
//! cs.edit("MyComponent", "b", ValueWrapper::String("Hello".to_string()));
//! ```
//!
//! After producing a JavaScript module in `www/pkg` with
//! ```bash
//! wasm-pack build --out-dir www/pkg --release
//! ```
//! you can use the wrapper like this:
//!
//! ```javascript
//! let cs = wasm.make_cs();
//! cs.subscribe("MyComponent", "a",
//!     new_value => console.log("a =", new_value),
//!     () => console.log("a is pending"),
//!     err => console.log("a failed:", err)
//! );
//! cs.edit("MyComponent", "a", wasm.ValueWrapper.i32(5));
//! cs.edit("MyComponent", "b", wasm.ValueWrapper.String("Hello"));
//! cs.solve();
//! ```
//!
//! ## Multithreaded
//!
//! Remember to add the `thread` feature flag in your `Cargo.toml`.
//!
//! ```toml
//! hotdrink-wasm = { version = "0.1.1", features = ["thread"] }
//! ```
//!
//! To use a multithreaded constraint system, you would create it like this instead:
//!
//! ```rust
//! #[cfg(feature = "thread")]
//! use hotdrink_wasm::{constraint_system_wrapper_threaded, thread::{StaticPool, TerminationStrategy}};
//!
//! #[cfg(feature = "thread")]
//! constraint_system_wrapper_threaded!(
//!     // A wrapper around `ConstraintSystem`.
//!     pub struct MyCs {
//!         // A wrapper that generates functions
//!         // for each variant of the inner value.
//!         pub struct ValueWrapper {
//!             // The actual value type inside the `ConstraintSystem`.
//!             #[derive(Clone, Debug)]
//!             pub enum Value {
//!                 i32,
//!                 String
//!             }
//!         }
//!         thread_pool: StaticPool, // Or DynamicPool
//!         num_threads: 4,          // Number of threads
//!         termination_strategy: TerminationStrategy::UnusedResultAndNotDone
//!     }
//! );
//! ```
//!
//! To use Web Workers from Rust, the we must compile with `--target no-modules`.
//!
//! ```bash
//! wasm-pack build --out-dir www/pkg --target no-modules --release
//! ```
//!
//! This will produce WebAssembly code and JS wrappers in `www/pkg`, which can then be imported there.
//! See wasm-pack's documentation for more information.

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

pub mod event;
pub mod macros;
pub mod thread;
pub mod util;

#[cfg(feature = "demo")]
pub mod demo;
