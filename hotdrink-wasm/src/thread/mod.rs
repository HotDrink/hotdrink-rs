//! Thread pool implementations and wrappers around Web Workers to allow them to run Rust-functions.

mod pool;
pub mod worker;

pub use pool::{DynamicPool, PoolWorker, StaticPool, WorkerPool};
