use super::js_event::JsEvent;
use crate::thread::worker::generic_worker::GenericWorker;
use js_sys::Function;
use std::sync::mpsc::{self};
use wasm_bindgen::JsValue;

pub struct EventListener<T, E> {
    worker: GenericWorker,
    sender: mpsc::Sender<JsEvent<T, E>>,
}

impl<T, E> EventListener<T, E>
where
    T: Send + 'static,
    E: Send + 'static,
{
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

    pub fn listen(&self, callback: &Function) {
        self.worker.on_message(&callback);
    }

    pub fn sender(&self) -> &mpsc::Sender<JsEvent<T, E>> {
        &self.sender
    }

    pub fn terminate(&self) {
        self.worker.terminate();
    }
}
