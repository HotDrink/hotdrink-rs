use hotdrink_rs::{examples::components::numbers::sum, model::Component, Event};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub struct IntComponent {
    inner: Component<i32>,
}

#[no_mangle]
pub extern "C" fn component_new() -> *mut IntComponent {
    let sum: Component<i32> = sum();
    Box::into_raw(Box::new(IntComponent { inner: sum }))
}

/// Update the specified component.
///
/// # Safety
///
/// The argument must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn component_free(component: *mut IntComponent) {
    Box::from_raw(component);
}

#[repr(C)]
pub enum CEventType {
    Pending,
    Ready,
    Error,
}

#[repr(C)]
pub union CEventData {
    value: i32,
    error: *const c_char,
}

#[repr(C)]
pub struct CEvent {
    variable: *const c_char,
    event_type: CEventType,
    event_data: CEventData,
}

/// Subscribe to the specified component.
///
/// # Safety
///
/// The arguments must be valid pointers.
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

/// Set a variable's value in the specified component.
///
/// # Safety
///
/// The arguments must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn component_set_variable(
    component: *mut IntComponent,
    variable: *mut c_char,
    value: i32,
) {
    let variable = CStr::from_ptr(variable).to_str().unwrap();
    (*component).inner.set_variable(variable, value).unwrap();
}

/// Update the specified component.
///
/// # Safety
///
/// The argument must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn component_update(component: *mut IntComponent) {
    (*component).inner.update().unwrap();
}
