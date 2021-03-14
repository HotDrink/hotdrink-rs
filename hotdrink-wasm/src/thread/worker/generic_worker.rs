// Based on https://github.com/rustwasm/wasm-bindgen/blob/master/examples/raytrace-parallel/src/pool.rs

// Silences warnings from the compiler about Work.func and child_entry_point
// being unused when the target is not wasm.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

//! A module for creating Web Workers that run Rust closures in a separate thread.

use crate::thread::worker::worker_script;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DedicatedWorkerGlobalScope, Worker, WorkerOptions};

/// The work to be done by the web worker.
/// It is given access to the global scope in order to send messages back to it.
struct Work {
    func: Box<dyn FnOnce(DedicatedWorkerGlobalScope) + Send>,
}

/// Spawns a new worker.
///
/// # Errors
///
/// Returns any error that may happen while a JS web worker is created
/// and a message is sent to it.
fn spawn_worker(name: &str, wasm_bindgen_shim_url: Option<&str>) -> Result<Worker, JsValue> {
    // Set the name of the worker
    let mut options = WorkerOptions::new();
    options.name(&name);

    // Either use the input-url, or create a new blob
    let worker = match wasm_bindgen_shim_url {
        Some(url) => Worker::new_with_options(&url, &options)?,
        None => Worker::new_with_options(&worker_script::create(), &options)?,
    };

    // With a worker spun up send it the module/memory so it can start
    // instantiating the wasm module. Later it might receive further
    // messages about code to run on the wasm module.
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::module());
    array.push(&wasm_bindgen::memory());
    worker.post_message(&array)?;

    Ok(worker)
}

#[wasm_bindgen]
pub struct GenericWorker {
    worker: Worker,
}

impl GenericWorker {
    pub fn new(name: &str) -> Result<GenericWorker, JsValue> {
        Ok(Self {
            worker: spawn_worker(name, None)?,
        })
    }

    pub fn from_url(name: &str, wasm_bindgen_shim_url: &str) -> Result<GenericWorker, JsValue> {
        Ok(Self {
            worker: spawn_worker(name, Some(wasm_bindgen_shim_url))?,
        })
    }
}

#[wasm_bindgen]
impl GenericWorker {
    #[wasm_bindgen]
    pub fn on_message(&self, f: &Function) {
        self.worker.set_onmessage(Some(&f));
    }

    #[wasm_bindgen]
    pub fn terminate(&self) {
        self.worker.terminate();
    }
}

impl GenericWorker {
    pub fn execute(
        &self,
        f: Box<dyn FnOnce(DedicatedWorkerGlobalScope) + Send>,
    ) -> Result<(), JsValue> {
        let worker = &self.worker;
        let work = Box::new(Work { func: Box::new(f) });
        let ptr = Box::into_raw(work);
        match worker.post_message(&JsValue::from(ptr as u32)) {
            Ok(()) => Ok(()),
            Err(e) => {
                unsafe {
                    drop(Box::from_raw(ptr));
                }
                Err(e)
            }
        }
    }
}

/// To be called from JS.
/// Pass in a pointer to the work to be executed, and pass in the global worker scope.
/// This is so that the work can call post_message if it wants the Web Worker to
/// pass some data back to the subscriber.
#[wasm_bindgen]
pub fn generic_worker_entry_point(ptr: u32) -> Result<(), JsValue> {
    let ptr = unsafe { Box::from_raw(ptr as *mut Work) };
    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    (ptr.func)(global);
    Ok(())
}
