//! A constraint system that only contains strings.

use hotdrink_wasm::thread::{TerminationStrategy, StaticPool};

hotdrink_wasm::component_type_wrapper! {
    pub struct StringWrapper {
        #[derive(Debug, Clone)]
        pub enum StringValue {
            String
        }
    }
}

hotdrink_wasm::constraint_system_wrapper_threaded!(
    StringJsCs,
    StringWrapper,
    StringValue,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);
