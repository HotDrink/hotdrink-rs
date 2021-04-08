//! A constraint system that only contains strings.

use crate::thread::StaticPool;
use hotdrink_rs::thread::TerminationStrategy;

crate::component_type_wrapper! {
    pub struct StringWrapper {
        #[derive(Debug, Clone)]
        pub enum StringValue {
            String
        }
    }
}

crate::constraint_system_wrapper!(
    StringJsCs,
    StringWrapper,
    StringValue,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);
