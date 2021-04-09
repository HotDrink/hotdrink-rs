//! A constraint system that only contains numbers.

use crate::thread::StaticPool;
use hotdrink_rs::thread::TerminationStrategy;

crate::component_type_wrapper! {
    pub struct I32Wrapper {
        #[derive(Debug, Clone)]
        pub enum I32 {
            i32
        }
    }
}

crate::constraint_system_wrapper!(
    I32JsCs,
    I32Wrapper,
    I32,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);

crate::component_type_wrapper! {
    pub struct F64Wrapper {
        #[derive(Debug, Clone)]
        pub enum F64 {
            f64
        }
    }
}

crate::constraint_system_wrapper!(
    F64JsCs,
    F64Wrapper,
    F64,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);

crate::component_type_wrapper! {
    pub struct NumberWrapper {
        #[derive(Debug, Clone)]
        pub enum Number {
            i32,
            f64
        }
    }
}

crate::constraint_system_wrapper!(
    NumberJsCs,
    NumberWrapper,
    Number,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);
