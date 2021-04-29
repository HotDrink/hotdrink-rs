//! A library that shows how C/C++ bindings can be created for `hotdrink-rs`.

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    missing_docs
)]

use hotdrink_rs::{examples::components::numbers::sum, model::Component, Event};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// An opaque struct that holds a Component<i32>.
/// Component<i32> can not be exported on its own,
/// so we must wrap it.
#[derive(Debug)]
pub struct IntComponent {
    inner: Component<i32>,
}

/// Creates a new sum-component.
#[no_mangle]
pub extern "C" fn component_new() -> *mut IntComponent {
    let sum: Component<i32> = sum();
    Box::into_raw(Box::new(IntComponent { inner: sum }))
}

/// Calls update on the specified component.
///
/// # Safety
///
/// The argument must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn component_free(component: *mut IntComponent) {
    Box::from_raw(component);
}

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
#[repr(C)]
#[derive(Copy, Clone)]
pub union CEventData {
    value: i32,
    error: *const c_char,
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

/// Subscribes to the specified component.
///
/// # Safety
///
/// The component argument must be valid pointer,
/// and the variable must be a valid nul-terminated string.
#[no_mangle]
pub unsafe extern "C" fn component_subscribe(
    component: *mut IntComponent,
    variable: *mut c_char,
    callback: extern "C" fn(CEvent),
) -> bool {
    let variable = CStr::from_ptr(variable).to_str().unwrap();
    let variable_clone = variable.to_owned();
    (*component)
        .inner
        .subscribe(variable, move |e| {
            let variable = CString::new(variable_clone.clone()).unwrap();
            let error = CString::new("error").unwrap();
            let error = error.as_ptr();
            let c_event: CEvent = match e {
                Event::Pending => CEvent {
                    variable: variable.as_ptr(),
                    event_type: CEventType::Pending,
                    event_data: CEventData { value: 0 },
                },
                Event::Ready(&value) => CEvent {
                    variable: variable.as_ptr(),
                    event_type: CEventType::Ready,
                    event_data: CEventData { value },
                },
                Event::Error(_) => CEvent {
                    variable: variable.as_ptr(),
                    event_type: CEventType::Ready,
                    event_data: CEventData { error },
                },
            };
            callback(c_event);
        })
        .is_ok()
}

/// Sets a variable's value in the specified component.
///
/// # Safety
///
/// The component component must be valid pointer,
/// and the variable must be a valid nul-terminated string.
#[no_mangle]
pub unsafe extern "C" fn component_set_variable(
    component: *mut IntComponent,
    variable: *mut c_char,
    value: i32,
) {
    let variable = CStr::from_ptr(variable).to_str().unwrap();
    (*component).inner.set_variable(variable, value).unwrap();
}

/// Updates the specified component.
///
/// # Safety
///
/// The argument must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn component_update(component: *mut IntComponent) {
    (*component).inner.update().unwrap();
}
