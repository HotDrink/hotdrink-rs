//! A macro that generates a multithreaded wrapper for [`ConstraintSystem`](hotdrink_rs::model::ConstraintSystem) that can be compiled to WebAssembly.

/// A macro that generates a multithreaded wrapper for [`ConstraintSystem`](hotdrink_rs::model::ConstraintSystem) that can be compiled to WebAssembly.
///
/// By providing an identifier, wrapper type, and inner type (the two last generated with [`component_type_wrapper!`]($crate::component_type_wrapper!)),
/// it will automatically generate a wrapper that can be returned to and used from JavaScript.
///
/// # Examples
///
/// ```rust
/// use hotdrink_rs::{executor::DummyExecutor, model::ConstraintSystem};
/// use hotdrink_wasm::thread::TerminationStrategy;
///
/// hotdrink_wasm::constraint_system_wrapper_threaded!(
///     pub struct ConstraintSystemWrapper {
///         pub struct InnerTypeWrapper {
///             #[derive(Clone, Debug)]
///             pub enum InnerType {
///                 i32,
///                 String
///             }
///         }
///         thread_pool: DummyExecutor,
///         num_threads: 4,
///         termination_strategy: TerminationStrategy::UnusedResultAndNotDone
///     }
/// );
/// ```
#[macro_export]
macro_rules! constraint_system_wrapper_threaded {
    (
        $vis:vis struct $cs_name:ident {
            $(#[$outer_meta:meta])*
            $wrapper_vis:vis struct $wrapper_type:ident {
                $(#[$inner_meta:meta])*
                $inner_vis:vis enum $inner_type:ident { $( $constr:ident ),* }
            }
            thread_pool: $thread_pool_type:ty,
            num_threads: $num_threads:expr,
            termination_strategy: $termination_strategy:expr $(,)?
        }
    ) => {
        $crate::wrap_enum! {
            $(#[$outer_meta])*
            $wrapper_vis struct $wrapper_type {
                $(#[$inner_meta])*
                $inner_vis enum $inner_type { $( $constr ),* }
            }
        }
        /// A wrapper around the internal constraint system.
        /// A macro is used to construct the type that the library user wants,
        /// since `wasm_bindgen` requires a concrete type.
        #[wasm_bindgen::prelude::wasm_bindgen]
        #[allow(missing_debug_implementations)]
        pub struct $cs_name {
            inner: std::sync::Mutex<hotdrink_rs::model::ConstraintSystem<$inner_type>>,
            event_listener: $crate::event::event_listener::EventListener<
                $inner_type,
                hotdrink_rs::solver::SolveError,
            >,
            event_handler: std::sync::Mutex<
                $crate::event::event_handler::EventHandler<
                    $inner_type,
                    hotdrink_rs::solver::SolveError,
                >,
            >,
            pool: std::sync::Mutex<$thread_pool_type>,
        }

        impl $cs_name {
            /// Wraps an existing constraint system,
            /// and initializes the struct.
            pub fn wrap(
                inner: hotdrink_rs::model::ConstraintSystem<$inner_type>,
            ) -> Result<$cs_name, wasm_bindgen::JsValue> {
                // Create the worker script blob
                let worker_script_url = $crate::thread::worker::worker_script::create();

                // Create the event listener
                let event_listener =
                    $crate::event::event_listener::EventListener::from_url(&worker_script_url)?;
                // Create the worker pool for executing methods
                let pool: $thread_pool_type = $crate::thread::WebWorkerPool::from_url(
                    $num_threads,
                    $termination_strategy,
                    &worker_script_url,
                )?;
                // Combine it all
                Ok(Self {
                    inner: std::sync::Mutex::new(inner),
                    event_listener,
                    event_handler: std::sync::Mutex::new(
                        $crate::event::event_handler::EventHandler::new(),
                    ),
                    pool: std::sync::Mutex::new(pool),
                })
            }
        }

        impl Drop for $cs_name {
            fn drop(&mut self) {
                self.event_listener.terminate();
            }
        }

        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $cs_name {
            /// Decide what to do with all the events from the constraint system,
            /// and perform the initial solve once that is done.
            pub fn listen(&mut self, cb: js_sys::Function) {
                self.event_listener.listen(&cb);
            }

            /// Sets the callbacks to call when the specified variable changes state.
            /// `on_pending` should not have any parameters, `on_ready` will receive the new value,
            /// and `on_error` will receive information about the error.
            pub fn subscribe(
                &self,
                component: &str,
                variable: &str,
                on_ready: Option<js_sys::Function>,
                on_pending: Option<js_sys::Function>,
                on_error: Option<js_sys::Function>,
                on_ok: Option<js_sys::Function>,
            ) {
                {
                    let mut event_handler = self.event_handler.lock().unwrap();
                    if let Some(on_ready) = on_ready {
                        event_handler.set_on_ready(component, variable, on_ready);
                    }
                    if let Some(on_pending) = on_pending {
                        event_handler.set_on_pending(component, variable, on_pending);
                    }
                    if let Some(on_error) = on_error {
                        event_handler.set_on_error(component, variable, on_error);
                    }
                    if let Some(on_ok) = on_ok {
                        event_handler.set_on_ok(component, variable, on_ok);
                    }
                }
                {
                    let component = component.to_owned();
                    let variable = variable.to_owned();
                    let mut inner = self.inner.lock().unwrap();
                    let sender = self.event_listener.sender().clone();
                    let sender = std::sync::Mutex::new(sender);
                    let (component_clone, variable_clone) = (component.clone(), variable.clone());
                    let result = inner.subscribe(&component_clone, &variable_clone, move |e| {
                        let js_event = $crate::event::js_event::JsEvent::new(
                            component.clone(),
                            variable.clone(),
                            e.into(),
                        );
                        sender.lock().unwrap().send(js_event).unwrap()
                    });

                    if let Err(e) = result {
                        log::error!("Subscribe failed: {}", e);
                    }
                }
            }

            /// Removes the callbacks from the specified variable.
            pub fn unsubscribe(&self, component: &str, variable: &str) {
                {
                    let mut event_handler = self.event_handler.lock().unwrap();
                    event_handler.unsubscribe(component, variable);
                }
                {
                    let mut inner = self.inner.lock().unwrap();
                    if let Err(e) = inner.unsubscribe(component, variable) {
                        log::error!("Unsubscribe failed: {}", e);
                    }
                }
            }

            /// Notifies the constraint system of an event, such as a thread having updated a value.
            pub fn notify(&self, event_ptr: u32) {
                use hotdrink_rs::solver::SolveError;
                use $crate::event::js_event::JsEvent;
                let event =
                    unsafe { Box::from_raw(event_ptr as *mut JsEvent<$inner_type, SolveError>) };
                let mut event_handler = self.event_handler.lock().unwrap();
                if let Err(e) = event_handler.handle_event(*event) {
                    log::error!("Failed to handle event: {:?}", e);
                };
            }

            /// Attempts to re-enforce all constraints.
            pub fn solve(&self) {
                let mut inner = self.inner.lock().unwrap();
                let pool = self.pool.lock().unwrap();
                if let Err(e) = inner.par_solve(&*pool) {
                    log::error!("Update failed: {:?}", e);
                }
            }

            /// Gives the specified variable a new value.
            pub fn edit(&self, component: &str, variable: &str, value: $wrapper_type) {
                let value = value.unwrap();
                self.inner
                    .lock()
                    .unwrap()
                    .edit(component, variable, value)
                    .unwrap_or_else(|e| {
                        log::error!("Could not set variable: {}", e);
                    });
            }

            /// Pins the specified variable, stopping it from changing.
            /// Note that this can cause the system to be overconstrained.
            pub fn pin(&self, component: &str, variable: &str) {
                self.inner
                    .lock()
                    .unwrap()
                    .pin(component, variable)
                    .unwrap_or_else(|e| {
                        log::error!("Could not pin variable: {}", e);
                    });
            }

            /// Unpins the specified variable, allowing it to change again.
            pub fn unpin(&self, component: &str, variable: &str) {
                self.inner
                    .lock()
                    .unwrap()
                    .unpin(component, variable)
                    .unwrap_or_else(|e| {
                        log::error!("Could not unpin variable: {}", e);
                    });
            }

            /// Undo the last change.
            pub fn undo(&self) {
                self.inner.lock().unwrap().undo().unwrap_or_else(|e| {
                    log::error!("Undo failed: {}", e);
                })
            }

            /// Undo the last change.
            pub fn redo(&self) {
                self.inner.lock().unwrap().redo().unwrap_or_else(|e| {
                    log::error!("Redo failed: {}", e);
                })
            }

            /// Enables the specified constraint.
            pub fn enable_constraint(&self, component: &str, constraint: &str) {
                self.inner
                    .lock()
                    .unwrap()
                    .enable_constraint(component, constraint)
                    .unwrap_or_else(|e| {
                        log::error!("Could not enable constraint: {}", e);
                    });
            }

            /// Disables the specified constraint.
            pub fn disable_constraint(&self, component: &str, constraint: &str) {
                self.inner
                    .lock()
                    .unwrap()
                    .disable_constraint(component, constraint)
                    .unwrap_or_else(|e| {
                        log::error!("Could not disable constraint: {}", e);
                    });
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::thread::TerminationStrategy;
    use hotdrink_rs::executor::DummyExecutor;

    #[ignore = "Simply for verification that it compiles"]
    #[test]
    fn it_compiles() {
        // Generate a JS wrapper for the constraint system
        crate::constraint_system_wrapper_threaded!(
            pub struct ConstraintSystemWrapper {
                pub struct InnerTypeWrapper {
                    #[derive(Clone, Debug)]
                    pub enum InnerType {
                        i32,
                        f64
                    }
                }
                thread_pool: DummyExecutor,
                num_threads: 4,
                termination_strategy: TerminationStrategy::UnusedResultAndNotDone
            }
        );
    }
}
