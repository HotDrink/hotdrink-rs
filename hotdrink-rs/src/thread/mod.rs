//! Thread pool traits and implementations.

mod dummy_pool;
mod thread_pool;

pub use dummy_pool::DummyPool;
pub use thread_pool::{TerminationHandle, ThreadPool};
