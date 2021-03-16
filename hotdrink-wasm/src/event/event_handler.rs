//! A module for creating event handlers for the constraint system.
//!
//! To use it, first construct a new `EventHandler`, add the desired callbacks,
//! then pass in the `handle_event` function to the constraint system.
//!
//! # Examples
//!
//! ```javascript
//! let cs = ...;
//! let event_handler = new EventHandler();
//! event_handler.subscribe("mycomp", "myvar");
//! cs.add_event_handler(event_handler.handle_event);
//! ```

use crate::event::js_event::JsEvent;
use hotdrink_rs::event::{Event, Identifier};
use itertools::Itertools;
use js_sys::Function;
use std::fmt::Debug;
use std::{collections::HashMap, fmt::Display, marker::PhantomData};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// A callback for handling events.
#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JsCallback {
    on_pending: Option<Function>,
    on_ready: Option<Function>,
    on_error: Option<Function>,
}

/// The main event handler containing the callbacks for variables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventHandler<T, E> {
    /// The actions to perform when a given variable is updated.
    callbacks: HashMap<Identifier, JsCallback>,
    /// To lock the event type
    phantom_data: PhantomData<(T, E)>,
}

impl<T, E> Default for EventHandler<T, E> {
    fn default() -> Self {
        Self {
            callbacks: HashMap::new(),
            phantom_data: PhantomData,
        }
    }
}

impl<T: Into<JsValue> + Debug, E: Display + Debug> EventHandler<T, E> {
    /// Construct a new event handler that reacts to no events.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the callback to call when the specified variable starts waiting for a new value.
    pub fn set_on_pending(
        &mut self,
        component: &str,
        variable: &str,
        on_pending: Function,
    ) -> Option<()> {
        let id = Identifier::new(component, variable);
        let callbacks = self.callbacks.entry(id).or_insert_with(JsCallback::default);
        callbacks.on_pending = Some(on_pending);
        Some(())
    }

    /// Sets the callback to call when the specified variable succeeds in getting a new value.
    pub fn set_on_ready(
        &mut self,
        component: &str,
        variable: &str,
        on_ready: Function,
    ) -> Option<()> {
        let id = Identifier::new(component, variable);
        let callbacks = self.callbacks.entry(id).or_insert_with(JsCallback::default);
        callbacks.on_ready = Some(on_ready);
        Some(())
    }

    /// Sets the callback to call when the specified variable fails to get a new value.
    pub fn set_on_error(
        &mut self,
        component: &str,
        variable: &str,
        on_error: Function,
    ) -> Option<()> {
        let id = Identifier::new(component, variable);
        let callbacks = self.callbacks.entry(id).or_insert_with(JsCallback::default);
        callbacks.on_error = Some(on_error);
        Some(())
    }

    /// Removes the callback for a specific variable from the event handler.
    pub fn unsubscribe(&mut self, component: &str, variable: &str) {
        let id = Identifier::new(component, variable);
        if self.callbacks.remove(&id).is_none() {
            log::error!(
                "Attempted to unsubscribe from {}.{} before subscribing",
                component,
                variable
            );
        }
    }

    /// Handles an event based on the attached callbacks.
    pub fn handle_event(&mut self, e: JsEvent<T, E>) -> Result<(), JsValue> {
        let id = Identifier::new(&e.get_component(), &e.get_variable());
        // Apply the matching callback if one exists
        if let Some(cb) = self.callbacks.get(&id) {
            match e.get_event() {
                Event::Pending => {
                    if let Some(on_pending) = &cb.on_pending {
                        on_pending.call0(&JsValue::null())?;
                    }
                }
                Event::Ready(value) => {
                    if let Some(on_ready) = &cb.on_ready {
                        let js_value: JsValue = value.into();
                        on_ready.call1(&JsValue::null(), &js_value)?;
                    }
                }
                Event::Error(error) => {
                    if let Some(on_error) = &cb.on_error {
                        let err_msg = error.iter().map(|e| format!("{}", e)).join("\r\n");
                        on_error.call1(&JsValue::null(), &JsValue::from(err_msg))?;
                    }
                }
            }
        }
        Ok(())
    }
}
