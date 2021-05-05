//! A constraint system example that shows off multiple features.

use hotdrink_rs::{
    component, fail, model::ConstraintSystem, ret, thread::TerminationStrategy, util::fib::slow_fib,
};
use hotdrink_wasm::thread::StaticPool;
use wasm_bindgen::prelude::wasm_bindgen;

hotdrink_wasm::component_type_wrapper! {
    pub struct I32OrStringWrapper {
        #[derive(Debug, Clone)]
        pub enum I32OrString {
            i32,
            String
        }
    }
}

hotdrink_wasm::constraint_system_wrapper_threaded!(
    I32OrStringCs,
    I32OrStringWrapper,
    I32OrString,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);

/// Constructs a new [`ConstraintSystem<MyValue>`].
pub fn responsive_inner() -> ConstraintSystem<I32OrString> {
    let a = component! {
        component A {
            let a: String = "Hello", a_length: i32 = 0, a_fib: i32 = 0;
            constraint A1 {
                a(a: &String) -> [a_length] = {
                    ret![5 * a.len() as i32]
                };
            }
            constraint A2 {
                a(a_length: &i32) -> [a_fib] = {
                    ret![slow_fib(*a_length)]
                };
            }
        }
    };

    let b = component! {
        component B {
            let b: String = "Rudi", b_age: i32 = 0, b_fib: i32 = 0;
            constraint B1 {
                a(b: &String) -> [b_age] = {
                    slow_fib(42);
                    match b.as_str() {
                        "" => ret![30],
                        "Rudi" => ret![24],
                        "Rust" => ret![43],
                        _ => fail!("Lookup failure, could not find `{}`", b),
                    }
                };
            }
            constraint B2 {
                a(b_age: &i32) -> [b_fib] = {
                    ret![slow_fib(*b_age)]
                };
            }
        }
    };

    let c = component! {
        component C {
            let c: i32 = 0, c_out: i32 = 0;
            constraint C {
                a(c: &i32) -> [c_out] = ret![2*c];
                b(c_out: &i32) -> [c] = ret![c_out/2];
            }
        }
    };

    // Add the component to the constraint system
    let mut cs = ConstraintSystem::new();
    cs.add_component(a);
    cs.add_component(b);
    cs.add_component(c);
    cs
}

/// Wraps a [`ConstraintSystem<MyValue>`] so that it can be used from JavaScript.
#[wasm_bindgen]
pub fn responsive() -> I32OrStringCs {
    I32OrStringCs::wrap(responsive_inner()).unwrap()
}
