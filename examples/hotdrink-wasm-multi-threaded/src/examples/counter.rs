//! A constraint system example that increments a value each update of the system.

use hotdrink_rs::{
    component,
    model::ConstraintSystem,
    ret,
    thread::{DummyPool, TerminationStrategy},
};
use wasm_bindgen::prelude::wasm_bindgen;

// Generate the value type.
hotdrink_wasm::component_type_wrapper! {
    pub struct CounterValueWrapper {
        #[derive(Debug, Clone)]
        pub enum CounterValue {
            i32
        }
    }
}

// Generate the constraint system wrapper.
hotdrink_wasm::constraint_system_wrapper_threaded!(
    CounterConstraintSystem,
    CounterValueWrapper,
    CounterValue,
    DummyPool,
    1,
    TerminationStrategy::UnusedResultAndNotDone
);

/// Constructs a [`CounterConstraintSystem`].
#[wasm_bindgen]
pub fn counter_cs() -> CounterConstraintSystem {
    let mut cs = ConstraintSystem::new();
    cs.add_component(component! {
        component Counter {
            let count: i32 = 0;
            constraint Inc {
                inc(count: &i32) -> [count] = ret![*count + 1];
            }
        }
    });
    CounterConstraintSystem::wrap(cs).expect("Could not create constraint system")
}
