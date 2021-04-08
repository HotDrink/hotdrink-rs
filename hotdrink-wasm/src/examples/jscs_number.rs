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
    NumberJsCs,
    I32Wrapper,
    I32,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);
