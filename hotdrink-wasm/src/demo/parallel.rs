//! A module for testing new features for `hotdrink-rs`.

#[cfg(feature = "demo")]
use crate::thread::{StaticPool, TerminationStrategy};
#[cfg(feature = "demo")]
use hotdrink_rs::{component, model::ConstraintSystem, ret, util::fib::slow_fib};
#[cfg(feature = "demo")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[cfg(feature = "demo")]
crate::constraint_system_wrapper_threaded! {
    pub struct CsWrapper {
        pub struct ValueWrapper {
            #[derive(Clone, Debug)]
            pub enum Value {
                i32
            }
        }
        thread_pool: StaticPool,
        num_threads: 4,
        termination_strategy: TerminationStrategy::UnusedResultAndNotDone
    }
}

/// An example of how to return a constraint system to JavaScript.
#[cfg(feature = "demo")]
#[wasm_bindgen]
pub fn example_cs() -> Result<CsWrapper, JsValue> {
    let mut cs = ConstraintSystem::new();
    cs.add_component(component! {
        component A {
            let a: i32 = 0, b: i32 = 0, c: i32 = 0, d: i32 = 0,
                e: i32 = 0, f: i32 = 0, g: i32 = 0, h: i32 = 0, i: i32 = 0;
            constraint AB { a(a: &i32) -> [b] = { ret![slow_fib(*a)] }; }
            constraint AC { a(a: &i32) -> [c] = { ret![slow_fib(*a)] }; }
            constraint AD { a(a: &i32) -> [d] = { ret![slow_fib(*a)] }; }
            constraint AE { a(a: &i32) -> [e] = { ret![slow_fib(*a)] }; }
            constraint AF { a(a: &i32) -> [f] = { ret![slow_fib(*a)] }; }
            constraint AG { a(a: &i32) -> [g] = { ret![slow_fib(*a)] }; }
            constraint AH { a(a: &i32) -> [h] = { ret![slow_fib(*a)] }; }
            constraint AI { a(a: &i32) -> [i] = { ret![slow_fib(*a)] }; }
        }
    });
    CsWrapper::wrap(cs)
}
