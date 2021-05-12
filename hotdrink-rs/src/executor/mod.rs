//! A trait and implementations for method executors.

mod dummy_executor;
mod method_executor;

pub use dummy_executor::DummyExecutor;
pub use method_executor::{MethodExecutor, TerminationHandle};
