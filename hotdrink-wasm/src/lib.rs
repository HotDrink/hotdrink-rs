//! HotDrink implemented using Rust and WebAssembly.
//!
//! # Prerequisites
//!
//! Begin by downloading the prerequisites below.
//!
//! * Rust (nightly)
//! * `wasm-pack`
//!
//! To use Rust nightly for this project, either set it globally with `rustup default nightly`, or for this project with `rustup override set nightly`.
//! It is required for the experimental benchmarking features.
//!
//! # Build
//!
//! The standard library must be recompiled to use Web Workers from Rust, and the WebAssembly must be compiled with `--target no-modules` to be imported by them.
//! To run the example in `www`, use the makefile to perform the steps above, then read the instructions in that directory.

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

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

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
