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
//! cs.set_variable("MyComponent", "a", ValueWrapper::i32(5));
//! cs.set_variable("MyComponent", "b", ValueWrapper::String("Hello".to_string()));
//! ```
//!
//! After producing a JavaScript module in www/pkg with
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
//! cs.set_variable("MyComponent", "a", wasm.ValueWrapper.i32(5));
//! cs.set_variable("MyComponent", "b", wasm.ValueWrapper.String("Hello"));
//! cs.update();
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
//! This will produce WebAssembly code and JS wrappers in www/pkg, which can then be imported there.
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

#[cfg(feature = "demo")]
use crate::thread::{StaticPool, TerminationStrategy};
#[cfg(feature = "demo")]
use hotdrink_rs::{component, model::ConstraintSystem, ret, util::fib::slow_fib};
#[cfg(feature = "demo")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod event;
pub mod macros;
pub mod thread;
pub mod util;

/// Check how long it takes for a web worker running Wasm to start.
#[cfg(feature = "demo")]
#[wasm_bindgen]
pub fn bench_web_worker_init() {
    use js_sys::Date;
    use thread::worker::generic_worker::GenericWorker;
    let start = Date::now();
    let worker = GenericWorker::new("TestWorker").unwrap();
    worker
        .execute(Box::new(move |_| {
            let end = Date::now();
            log::info!("Spawning web worker took {}ms", end - start);
        }))
        .unwrap();
}

#[cfg(feature = "demo")]
crate::constraint_system_wrapper_threaded! {
    pub struct CsWrapper {
        pub struct ValueWrapper {
            #[derive(Clone, Debug)]
            pub enum Value {
                i32
            }
        }
        thread_pool: StaticPool,
        num_threads: 4,
        termination_strategy: TerminationStrategy::UnusedResultAndNotDone
    }
}

/// An example of how to return a constraint system to JavaScript.
#[cfg(feature = "demo")]
#[wasm_bindgen]
pub fn example_cs() -> Result<CsWrapper, JsValue> {
    let mut cs = ConstraintSystem::new();
    cs.add_component(component! {
        component A {
            let a: i32 = 0, b: i32 = 0, c: i32 = 0, d: i32 = 0,
                e: i32 = 0, f: i32 = 0, g: i32 = 0, h: i32 = 0, i: i32 = 0;
            constraint AB { a(a: &i32) -> [b] = { ret![slow_fib(*a)] }; }
            constraint AC { a(a: &i32) -> [c] = { ret![slow_fib(*a)] }; }
            constraint AD { a(a: &i32) -> [d] = { ret![slow_fib(*a)] }; }
            constraint AE { a(a: &i32) -> [e] = { ret![slow_fib(*a)] }; }
            constraint AF { a(a: &i32) -> [f] = { ret![slow_fib(*a)] }; }
            constraint AG { a(a: &i32) -> [g] = { ret![slow_fib(*a)] }; }
            constraint AH { a(a: &i32) -> [h] = { ret![slow_fib(*a)] }; }
            constraint AI { a(a: &i32) -> [i] = { ret![slow_fib(*a)] }; }
        }
    });
    CsWrapper::wrap(cs)
}
