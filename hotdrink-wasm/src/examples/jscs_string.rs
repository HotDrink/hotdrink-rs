//! A constraint system that only contains strings.

use crate::thread::pool::StaticPool;
use hotdrink_rs::thread::thread_pool::TerminationStrategy;

crate::gen_js_val! {
    pub StringWrapper {
        #[derive(Debug, Clone)]
        pub StringValue {
            String
        }
    }
}

crate::gen_js_constraint_system!(
    StringJsCs,
    StringWrapper,
    StringValue,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);
