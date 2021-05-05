//! Macros for generating constraint system wrappers that can be used from JavaScript,
//! and values that can be used within them.

pub mod constraint_system_wrapper;
#[cfg(feature = "thread")]
pub mod constraint_system_wrapper_threaded;
pub mod wrap_enum;
