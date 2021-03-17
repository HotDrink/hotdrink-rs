//! A wrapper library around `hotdrink-rs` for compilation to WebAssembly.
//!
//! # Prerequisites
//!
//! The project uses multiple nightly features, and must be built using nightly Rust.
//! I recommend using `rustup`, which can be downloaded [here](https://rustup.rs/),
//!
//! You also need `wasm-pack`, which can be downloaded [here](https://rustwasm.github.io/wasm-pack/installer/).
//!
//! The standard library must be recompiled, which means that we need the standard library source code.
//! This can be downloaded with `rustup component add rust-src`.
//!
//! # Build
//!
//! To use Web Workers from Rust, the we must compile with `--target no-modules`.
//! This should be as simple as running the following:
//!
//! ```bash
//! wasm-pack build --out-dir www/pkg --target no-modules --release
//! ```
//!
//! This will produce WebAssembly code and JS wrappers in www/pkg, which can then be imported there.

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
pub mod examples;
pub mod macros;
pub mod thread;
pub mod util;
pub mod utils;

use js_sys::Date;
use thread::worker::generic_worker::GenericWorker;
use wasm_bindgen::prelude::*;

/// A wrapper for calling `console_log` like `println!`.
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (
        #[allow(unused_unsafe)]
        unsafe {
            #[cfg(target_arch="wasm32")]
            crate::log(&format_args!($($t)*).to_string());
        }
    )
}

/// A wrapper for calling `console_error` like `eprintln!`.
#[macro_export]
macro_rules! console_error {
    ($($t:tt)*) => (
        #[allow(unused_unsafe)]
        unsafe {
            #[cfg(target_arch="wasm32")]
            crate::error(&format_args!($($t)*).to_string());
        }
    )
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Perform setup such as setting the panic hook for better error messages,
/// and initialize the Wasm logging library.
/// Note that this is called once per thread since they all initialize the WebAssembly.
#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();

    // Only initialize the logger once
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        wasm_logger::init(wasm_logger::Config::default());
    });
}

/// Add two numbers
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtract two numbers
#[wasm_bindgen]
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

/// Asynchronous example.
/// This will become a promise when called from JavaScript.
#[wasm_bindgen]
pub async fn async_greet() -> String {
    "Hello".to_string()
}

/// Check how long it takes for a web worker running Wasm to start.
#[wasm_bindgen]
pub fn bench_web_worker_init() {
    let start = Date::now();
    let worker = GenericWorker::new("TestWorker").unwrap();
    worker
        .execute(Box::new(move |_| {
            let end = Date::now();
            log::info!("Spawning web worker took {}ms", end - start);
        }))
        .unwrap();
}
