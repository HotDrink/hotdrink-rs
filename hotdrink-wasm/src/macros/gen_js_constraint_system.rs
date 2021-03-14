#[macro_export]
macro_rules! gen_js_constraint_system {
    ($cs_name:ident, $wrapper_type:ty, $inner_type:ty, $thread_pool_type:ty, $num_threads:expr, $termination_strategy:expr) => {
        /// A wrapper around the internal constraint system.
        /// A macro is used to construct the type that the library user wants,
        /// since `wasm_bindgen` requires a concrete type.
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub struct $cs_name {
            inner: std::sync::Mutex<
                hotdrink_rs::data::constraint_system::ConstraintSystem<$inner_type>,
            >,
            event_listener: crate::event::event_listener::EventListener<
                $inner_type,
                hotdrink_rs::data::solve_error::SolveError,
            >,
            event_handler: std::sync::Mutex<
                crate::event::event_handler::EventHandler<
                    $inner_type,
                    hotdrink_rs::data::solve_error::SolveError,
                >,
            >,
            pool: std::sync::Mutex<$thread_pool_type>,
        }

        impl $cs_name {
            /// Wraps an existing constraint system,
            /// and initializes the struct.
            pub fn wrap(
                inner: hotdrink_rs::data::constraint_system::ConstraintSystem<$inner_type>,
            ) -> Result<$cs_name, wasm_bindgen::JsValue> {
                // Create the worker script blob
                let worker_script_url = crate::thread::worker::worker_script::create();
                // Create the event listener
                let event_listener =
                    crate::event::event_listener::EventListener::from_url(&worker_script_url)?;
                // Create the worker pool for executing methods
                let pool = hotdrink_rs::thread::thread_pool::WorkerPool::from_url(
                    $num_threads,
                    $termination_strategy,
                    &worker_script_url,
                )?;
                // Combine it all
                Ok(Self {
                    inner: std::sync::Mutex::new(inner),
                    event_listener,
                    event_handler: std::sync::Mutex::new(
                        crate::event::event_handler::EventHandler::new(),
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
                    let sender = self.event_listener.sender().clone();
                    inner.subscribe(&component.clone(), &variable.clone(), move |e| {
                        let js_event = crate::event::js_event::JsEvent::new(
                            component.clone(),
                            variable.clone(),
                            e,
                        );
                        sender.send(js_event).unwrap()
                    });
                }
            }

            pub fn unsubscribe(&self, component: &str, variable: &str) {
                {
                    let mut event_handler = self.event_handler.lock().unwrap();
                    event_handler.unsubscribe(component, variable);
                }
                {
                    let mut inner = self.inner.lock().unwrap();
                    inner.unsubscribe(component, variable);
                }
            }

            /// Notifies the constraint system of an event, such as a thread having updated a value.
            pub fn notify(&self, event_ptr: u32) {
                use crate::event::js_event::JsEvent;
                use hotdrink_rs::data::solve_error::SolveError;
                let event =
                    unsafe { Box::from_raw(event_ptr as *mut JsEvent<$inner_type, SolveError>) };
                let mut event_handler = self.event_handler.lock().unwrap();
                if let Err(e) = event_handler.handle_event(*event) {
                    log::error!("Failed to handle event: {:?}", e);
                };
            }

            /// Runs the planner and solver to re-enforce all constraints.
            pub fn update(&self) {
                let mut inner = self.inner.lock().unwrap();
                let mut pool = self.pool.lock().unwrap();
                if let Err(e) = inner.par_update(&mut *pool) {
                    log::error!("Solve failed: {:?}", e);
                }
            }

            /// Gives the specified variable a new value.
            pub fn set_variable(&self, component: &str, variable: &str, value: $wrapper_type) {
                let value = value.unwrap();
                self.inner
                    .lock()
                    .unwrap()
                    .set_variable(component, variable, value);
            }

            pub fn pin(&self, component: &str, variable: &str) {
                self.inner.lock().unwrap().pin(component, variable);
            }

            pub fn unpin(&self, component: &str, variable: &str) {
                self.inner.lock().unwrap().unpin(component, variable);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use hotdrink_rs::{
        data::component::Component,
        thread::{dummy_pool::DummyPool, thread_pool::TerminationStrategy},
    };
    use wasm_bindgen::JsValue;

    #[ignore = "Simply for verification that it compiles"]
    #[test]
    fn it_compiles() {
        use hotdrink_rs::data::constraint_system::ConstraintSystem;

        // Generate constraint system value and a JS wrapper for it
        crate::gen_js_val! {
            pub Wrapper {
                #[derive(Clone, Debug)]
                pub Inner {
                    i32,
                    f64
                }
            }
        };

        // Generate a JS wrapper for the constraint system
        crate::gen_js_constraint_system!(
            System,
            Wrapper,
            Inner,
            DummyPool,
            4,
            TerminationStrategy::UnusedResultAndNotDone
        );

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
