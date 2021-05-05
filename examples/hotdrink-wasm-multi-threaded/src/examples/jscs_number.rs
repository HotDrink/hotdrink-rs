//! A constraint system that only contains numbers.

use hotdrink_rs::thread::{DummyPool, TerminationStrategy};
use hotdrink_wasm::thread::StaticPool;

hotdrink_wasm::component_type_wrapper! {
    pub struct I32Wrapper {
        #[derive(Debug, Clone)]
        pub enum I32 {
            i32
        }
    }
}

hotdrink_wasm::constraint_system_wrapper_threaded!(
    I32JsCs,
    I32Wrapper,
    I32,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);

hotdrink_wasm::component_type_wrapper! {
    pub struct F64Wrapper {
        #[derive(Debug, Clone)]
        pub enum F64 {
            f64
        }
    }
}

hotdrink_wasm::constraint_system_wrapper_threaded!(
    F64JsCs,
    F64Wrapper,
    F64,
    DummyPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);

hotdrink_wasm::component_type_wrapper! {
    pub struct NumberWrapper {
        #[derive(Debug, Clone)]
        pub enum Number {
            i32,
            f64
        }
    }
}

hotdrink_wasm::constraint_system_wrapper_threaded!(
    NumberJsCs,
    NumberWrapper,
    Number,
    StaticPool,
    1,
    TerminationStrategy::Never
);
