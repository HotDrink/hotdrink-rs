//! An event-type for passing to C.

use std::{ffi::CString, os::raw::c_char};

/// The different kinds of events.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum CEventType {
    /// A pending value.
    Pending,
    /// A completed value.
    Ready,
    /// A failed value.
    Error,
}

/// Data contained in an [`CEvent`].
/// If pending, there is no data,
/// and this is just set to 0.
#[repr(C)]
#[derive(Copy, Clone)]
pub union CEventData {
    /// A successful value.
    pub value: i32,
    /// An error message.
    pub error: *const c_char,
}

impl std::fmt::Debug for CEventData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CEventData")
    }
}

/// An event from a constraint system with the variable name included.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CEvent {
    variable: *const c_char,
    event_type: CEventType,
    event_data: CEventData,
}

impl CEvent {
    /// Create a pending-event with no data.
    pub fn pending(variable: &CString) -> Self {
        Self {
            variable: variable.as_ptr(),
            event_type: CEventType::Pending,
            event_data: CEventData { value: 0 },
        }
    }
    /// Create a ready-event with integer data.
    pub fn ready(variable: &CString, value: i32) -> Self {
        Self {
            variable: variable.as_ptr(),
            event_type: CEventType::Ready,
            event_data: CEventData { value },
        }
    }
    /// Create an error-event with string data.
    pub fn error(variable: &CString, error: &CString) -> Self {
        Self {
            variable: variable.as_ptr(),
            event_type: CEventType::Error,
            event_data: CEventData {
                error: error.as_ptr(),
            },
        }
    }
}
