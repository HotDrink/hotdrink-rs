//! Thread pool implementations.

#[cfg(feature = "thread")]
mod dynamic_pool;
#[cfg(feature = "thread")]
mod pool_worker;
#[cfg(feature = "thread")]
mod static_pool;

mod web_worker_pool;

#[cfg(feature = "thread")]
pub use dynamic_pool::DynamicPool;
#[cfg(feature = "thread")]
pub use pool_worker::PoolWorker;
#[cfg(feature = "thread")]
pub use static_pool::StaticPool;

pub use web_worker_pool::{TerminationStrategy, WebWorkerPool};
