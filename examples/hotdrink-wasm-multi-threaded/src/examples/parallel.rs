//! A constraint system example that shows off multiple features.

use super::jscs_number::{Number, NumberJsCs};
use hotdrink_rs::{component, model::ConstraintSystem, ret, util::fib::slow_fib};
use wasm_bindgen::prelude::wasm_bindgen;

/// Constructs a new [`ConstraintSystem<MyValue>`].
pub fn parallel_inner() -> ConstraintSystem<Number> {
    let a = component! {
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
    };

    // Add the component to the constraint system
    let mut cs = ConstraintSystem::new();
    cs.add_component(a);
    cs
}

/// Wraps a [`ConstraintSystem<MyValue>`] so that it can be used from JavaScript.
#[wasm_bindgen]
pub fn parallel() -> NumberJsCs {
    NumberJsCs::wrap(parallel_inner()).unwrap()
}
