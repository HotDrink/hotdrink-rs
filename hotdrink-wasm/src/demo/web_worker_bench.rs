//! A module for testing how long it takes to start a web worker.

use crate::thread::worker::generic_worker::GenericWorker;
use js_sys::Date;
use wasm_bindgen::prelude::wasm_bindgen;

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
