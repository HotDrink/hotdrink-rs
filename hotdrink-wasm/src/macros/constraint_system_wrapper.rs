//! A macro for generating a [`ConstraintSystem`](hotdrink_rs::model::ConstraintSystem) that can be compiled to WebAssembly.

/// A macro for generating a [`ConstraintSystem`](hotdrink_rs::model::ConstraintSystem) that can be compiled to WebAssembly.
///
/// By providing an identifier, wrapper type, inner type (the two last generated with [`component_type_wrapper!`]($crate::component_type_wrapper!)),
/// a thread pool implementation, the number of threads to use, and a termination strategy, it will automatically generate
/// a wrapper that can be returned to and used from JavaScript.
#[macro_export]
macro_rules! constraint_system_wrapper {
    ($cs_name:ident, $wrapper_type:ty, $inner_type:ty) => {
        /// A wrapper around the internal constraint system.
        /// A macro is used to construct the type that the library user wants,
        /// since `wasm_bindgen` requires a concrete type.
        #[wasm_bindgen::prelude::wasm_bindgen]
        #[allow(missing_debug_implementations)]
        pub struct $cs_name {
            inner: std::sync::Mutex<hotdrink_rs::model::ConstraintSystem<$inner_type>>,
            event_queue: std::sync::Arc<
                std::sync::Mutex<
                    std::collections::VecDeque<
                        $crate::event::js_event::JsEvent<
                            $inner_type,
                            hotdrink_rs::solver::SolveError,
                        >,
                    >,
                >,
            >,
            event_handler: std::sync::Mutex<
                $crate::event::event_handler::EventHandler<
                    $inner_type,
                    hotdrink_rs::solver::SolveError,
                >,
            >,
            pool: std::sync::Mutex<hotdrink_rs::thread::DummyPool>,
        }

        impl $cs_name {
            /// Wraps an existing constraint system,
            /// and initializes the struct.
            pub fn wrap(
                inner: hotdrink_rs::model::ConstraintSystem<$inner_type>,
            ) -> Result<$cs_name, wasm_bindgen::JsValue> {
                // Create the event listener
                // Create the worker pool for executing methods
                use hotdrink_rs::thread::ThreadPool;
                let pool = hotdrink_rs::thread::DummyPool::new(
                    1,
                    hotdrink_rs::thread::TerminationStrategy::Never,
                )?;
                // Combine it all
                Ok(Self {
                    inner: std::sync::Mutex::new(inner),
                    event_queue: Default::default(),
                    event_handler: std::sync::Mutex::new(
                        $crate::event::event_handler::EventHandler::new(),
                    ),
                    pool: std::sync::Mutex::new(pool),
                })
            }
        }

        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $cs_name {
            fn handle_events(&self) {
                // Call callbacks with initial events
                let mut event_handler = self.event_handler.lock().unwrap();
                let mut event_queue = self.event_queue.lock().unwrap();
                for event in event_queue.drain(..) {
                    if let Err(e) = event_handler.handle_event(event) {
                        log::error!("Failed to handle event: {:?}", e);
                    };
                }
                event_queue.clear();
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
                }
                {
                    let component = component.to_owned();
                    let variable = variable.to_owned();
                    let mut inner = self.inner.lock().unwrap();
                    let event_queue = std::sync::Arc::clone(&self.event_queue);
                    let (component_clone, variable_clone) = (component.clone(), variable.clone());
                    let result = inner.subscribe(&component_clone, &variable_clone, move |e| {
                        let js_event = $crate::event::js_event::JsEvent::new(
                            component.clone(),
                            variable.clone(),
                            e.into(),
                        );
                        event_queue.lock().unwrap().push_back(js_event);
                    });

                    if let Err(e) = result {
                        log::error!("Subscribe failed: {}", e);
                    }

                    self.handle_events();
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

            /// Runs the planner and solver to re-enforce all constraints.
            pub fn update(&self) {
                let mut inner = self.inner.lock().unwrap();
                let mut pool = self.pool.lock().unwrap();
                match inner.par_update(&mut *pool) {
                    Ok(()) => self.handle_events(),
                    Err(e) => {
                        log::error!("Update failed: {}", e);
                    }
                }
            }

            /// Gives the specified variable a new value.
            pub fn set_variable(&self, component: &str, variable: &str, value: $wrapper_type) {
                let value = value.unwrap();
                self.inner
                    .lock()
                    .unwrap()
                    .set_variable(component, variable, value)
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
                });
                self.handle_events();
            }

            /// Undo the last change.
            pub fn redo(&self) {
                self.inner.lock().unwrap().redo().unwrap_or_else(|e| {
                    log::error!("Redo failed: {}", e);
                });
                self.handle_events();
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
    use hotdrink_rs::model::Component;
    use wasm_bindgen::JsValue;

    #[ignore = "Simply for verification that it compiles"]
    #[test]
    fn it_compiles() {
        use hotdrink_rs::model::ConstraintSystem;

        // Generate constraint system value and a JS wrapper for it
        crate::component_type_wrapper! {
            pub struct Wrapper {
                #[derive(Clone, Debug)]
                pub enum Inner {
                    i32,
                    f64
                }
            }
        };

        // Generate a JS wrapper for the constraint system
        crate::constraint_system_wrapper!(System, Wrapper, Inner);

        let mut cs: ConstraintSystem<Inner> = ConstraintSystem::new();
        let comp: Component<Inner> = hotdrink_rs::component! {
            component empty_comp {
                let x: i32 = 0;
            }
        };
        cs.add_component(comp);
        let _jscs: Result<System, JsValue> = System::wrap(cs);
    }
}
