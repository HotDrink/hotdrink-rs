//! A struct for listening to events, and letting JavaScript react to them on the main thread.

use super::js_event::JsEvent;
use crate::thread::worker::generic_worker::GenericWorker;
use js_sys::Function;
use std::fmt::Debug;
use std::sync::mpsc::{self};
use wasm_bindgen::JsValue;

/// A struct that contains a [`GenericWorker`] that listens for events
/// from the internal channel.
///
/// This enables us to receive events from multiple threads that converge in a single channel,
/// then use postMessage to send these out from the worker and onto the main thread in JS.
/// So this is essentially just a way to get information back to the main thread.
pub struct EventListener<T, E> {
    worker: GenericWorker,
    sender: mpsc::Sender<JsEvent<T, E>>,
}

impl<T, E> Debug for EventListener<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventListener")
            .field("worker", &"...")
            .field("sender", &"...")
            .finish()
    }
}

impl<T, E> EventListener<T, E>
where
    T: Send + 'static,
    E: Send + 'static,
{
    /// Constructs a new `EventListener`, and makes a Web Worker that listens for events from the internal channel.
    pub fn from_url(wasm_bindgen_shim_url: &str) -> Result<Self, JsValue> {
        let (sender, receiver) = mpsc::channel();

        // Receive events from the constraint system,
        // and use post_message to send them to JS.
        let worker = GenericWorker::from_url("EventListener", wasm_bindgen_shim_url)?;
        worker
            .execute(Box::new(move |g: web_sys::DedicatedWorkerGlobalScope| {
                // For each event received
                for event in receiver {
                    let event_ptr = Box::into_raw(Box::new(event));
                    g.post_message(&JsValue::from(event_ptr as u32))
                        .expect("Could not send event pointer post message");
                }
            }))
            .expect("Could not start receiver");

        Ok(Self { worker, sender })
    }

    /// Sets the callback to call on values that are received from the Web Worker.
    pub fn listen(&self, callback: &Function) {
        self.worker.on_message(&callback);
    }

    /// Returns a reference to the sender.
    ///
    /// Use this to send messages to the worker.
    pub fn sender(&self) -> &mpsc::Sender<JsEvent<T, E>> {
        &self.sender
    }

    /// Terminate the worker.
    pub fn terminate(&self) {
        self.worker.terminate();
    }
}
