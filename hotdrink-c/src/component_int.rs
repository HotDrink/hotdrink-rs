//! A wrapper type around a component, as well as functions to be exported
//! to C to interact with the inner component.

use crate::c_event::CEvent;
use hotdrink_rs::{examples::components::numbers::sum, model::Component, Event};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// An opaque struct that holds a Component<i32>.
/// Component<i32> can not be exported on its own,
/// so we must wrap it.
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct Component_i32 {
    inner: Component<i32>,
}

/// Creates a new sum-component.
#[no_mangle]
pub extern "C" fn component_new() -> *mut Component_i32 {
    let sum: Component<i32> = sum();
    Box::into_raw(Box::new(Component_i32 { inner: sum }))
}

/// Calls update on the specified component.
///
/// # Safety
///
/// The argument must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn component_free(component: *mut Component_i32) {
    Box::from_raw(component);
}

/// Subscribes to the specified component.
///
/// # Safety
///
/// The component argument must be valid pointer,
/// and the variable must be a valid nul-terminated string.
#[no_mangle]
pub unsafe extern "C" fn component_subscribe(
    component: *mut Component_i32,
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
            let c_event: CEvent = match e {
                Event::Pending => CEvent::pending(&variable),
                Event::Ready(&value) => CEvent::ready(&variable, value),
                Event::Error(_) => CEvent::error(&variable, &error),
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
    component: *mut Component_i32,
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
pub unsafe extern "C" fn component_update(component: *mut Component_i32) {
    (*component).inner.update().unwrap();
}
