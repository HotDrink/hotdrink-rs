//! A constraint system example that increments a value each update of the system.

use hotdrink_rs::{
    component,
    model::ConstraintSystem,
    ret,
    thread::{dummy_pool::DummyPool, thread_pool::TerminationStrategy},
};
use wasm_bindgen::prelude::wasm_bindgen;

// Generate the value type.
crate::gen_js_val! {
    pub CounterValueWrapper {
        #[derive(Debug, Clone)]
        pub CounterValue {
            i32
        }
    }
}

// Generate the constraint system wrapper.
crate::gen_js_constraint_system!(
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
