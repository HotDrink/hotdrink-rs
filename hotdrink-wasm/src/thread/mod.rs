//! Thread pool implementations and wrappers around Web Workers to allow them to run Rust-functions.

mod pool;
#[cfg(feature = "thread")]
pub mod worker;

pub use pool::WorkerPool;
#[cfg(feature = "thread")]
pub use pool::{DynamicPool, PoolWorker, StaticPool};
