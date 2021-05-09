//! A module for demonstrations and testing,
//! not meant to be included when used by other crates.

pub mod parallel;
pub mod web_worker_bench;

/// Perform setup such as setting the panic hook for better error messages,
/// and initialize the Wasm logging library.
/// Note that this is called once per thread since they all initialize the WebAssembly.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Only initialize the logger once
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        wasm_logger::init(wasm_logger::Config::default());
    });
}
