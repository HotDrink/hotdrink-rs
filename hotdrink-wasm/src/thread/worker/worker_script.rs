//! From https://github.com/chemicstry/wasm_thread/blob/main/src/lib.rs

use wasm_bindgen::JsValue;
use web_sys::{Blob, Url};

/// Extracts path of the `wasm_bindgen` generated .js shim script
fn get_wasm_bindgen_shim_script_path() -> String {
    js_sys::eval(include_str!("./script_path.js"))
        .unwrap()
        .as_string()
        .unwrap()
}

/// Generates worker entry script as URL encoded blob
pub fn create() -> String {
    let wasm_bindgen_shim_url = get_wasm_bindgen_shim_script_path();

    // Generate script from template
    let template = include_str!("./generic_worker.js");
    let script = template.replace("WASM_BINDGEN_SHIM_URL", &wasm_bindgen_shim_url);

    // Create url encoded blob
    let arr = js_sys::Array::new();
    arr.set(0, JsValue::from_str(&script));
    let blob = Blob::new_with_str_sequence(&arr).unwrap();
    Url::create_object_url_with_blob(&blob).unwrap()
}
