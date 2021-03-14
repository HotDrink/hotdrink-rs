use crate::thread::pool::StaticPool;
use hotdrink_rs::thread::thread_pool::TerminationStrategy;

crate::gen_js_val! {
    pub I32Wrapper {
        #[derive(Debug, Clone)]
        pub I32 {
            i32
        }
    }
}

crate::gen_js_constraint_system!(
    NumberJsCs,
    I32Wrapper,
    I32,
    StaticPool,
    4,
    TerminationStrategy::UnusedResultAndNotDone
);
