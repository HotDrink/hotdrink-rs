//! A simple constraint system to use in benchmarks.

use hotdrink_rs::{
    examples::constraint_systems::make_empty_cs,
    thread::{DummyPool, TerminationStrategy},
};
use wasm_bindgen::prelude::wasm_bindgen;

/// A type with a single value.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Unit;

// The type of value to store in the constraint system.
crate::gen_js_val! {
    pub CsValueWrapper {
        #[derive(Debug, Clone)]
        pub CsValue {
            Unit
        }
    }
}

impl Default for CsValue {
    fn default() -> Self {
        CsValue::Unit(Unit)
    }
}

// A constraint system wrapper to access from JavaScript.
crate::gen_js_constraint_system!(
    BenchConstraintSystem,
    CsValueWrapper,
    CsValue,
    DummyPool,
    1,
    TerminationStrategy::UnusedResultAndNotDone
);

/// Constructs a [`BenchConstraintSystem`].
#[wasm_bindgen]
pub fn js_cs_empty() -> BenchConstraintSystem {
    BenchConstraintSystem::wrap(make_empty_cs(1, 100)).expect("Could not create constraint system")
}
