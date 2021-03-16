//! A Web Worker wrapper based on <https://github.com/rustwasm/wasm-bindgen/blob/master/examples/raytrace-parallel/src/pool.rs>.
//!
//! The main idea is to first [`Box`] a Rust-function, and then using [`Box::into_raw`] to get a raw pointer to it.
//! This pointer (a simple `u32`) is then sent to the worker (thread) using [`Worker::post_message`].
//!
//! The worker (which now operates on a different thread) can then call [`generic_worker_entry_point`],
//! and pass in the pointer. The pointer is then turned back into a function with [`Box::from_raw`] (which is unsafe),
//! and if it is a valid pointer, it will be called.
//!
//! Since [`generic_worker_entry_point`] was called from a Web Worker, the function itself is also run on a separate thread.

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
pub struct Work {
    func: Box<dyn FnOnce(DedicatedWorkerGlobalScope) + Send>,
}

impl std::fmt::Debug for Work {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Work").finish()
    }
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

/// A wrapper around a [`Worker`].
#[wasm_bindgen]
#[derive(Debug)]
pub struct GenericWorker {
    worker: Worker,
}

impl GenericWorker {
    /// Constructs a new `GenericWorker`.
    pub fn new(name: &str) -> Result<GenericWorker, JsValue> {
        Ok(Self {
            worker: spawn_worker(name, None)?,
        })
    }

    /// Constructs a new `GenericWorker` that runs the specified script.
    pub fn from_url(name: &str, wasm_bindgen_shim_url: &str) -> Result<GenericWorker, JsValue> {
        Ok(Self {
            worker: spawn_worker(name, Some(wasm_bindgen_shim_url))?,
        })
    }
}

#[wasm_bindgen]
impl GenericWorker {
    /// Sets the callback to call when the worker sends a message.
    #[wasm_bindgen]
    pub fn on_message(&self, f: &Function) {
        self.worker.set_onmessage(Some(&f));
    }

    /// Terminates the inner worker.
    #[wasm_bindgen]
    pub fn terminate(&self) {
        self.worker.terminate();
    }
}

impl GenericWorker {
    /// Executes the specified function in the Web Worker.
    ///
    /// This works by first [`Box`]-ing the function, and then using [`Box::into_raw`] to
    /// get a raw pointer to the function.
    /// This pointer (a simple `u32`) is then sent to the worker (thread).
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

/// This should only be called from the Web Worker script.
///
/// A Web Worker (which runs its script on a different thread) can call [`generic_worker_entry_point`]
/// and pass in a pointer to a function. The pointer is then turned back into a function with [`Box::from_raw`] (which is unsafe).
/// The function can then be executed, and if it uses the [`DedicatedWorkerGlobalScope`], it can also send data back out of the worker
/// to any subscribers of the worker.
///
/// # Safety
///
/// The pointer must be a valid pointer to some [`Work`].
#[wasm_bindgen]
pub fn generic_worker_entry_point(ptr: u32) -> Result<(), JsValue> {
    let ptr = unsafe { Box::from_raw(ptr as *mut Work) };
    let global = js_sys::global().unchecked_into::<DedicatedWorkerGlobalScope>();
    (ptr.func)(global);
    Ok(())
}
