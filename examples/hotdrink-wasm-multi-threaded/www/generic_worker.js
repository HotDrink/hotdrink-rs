// Hardcoded version of the worker script from hd_wasm::thread::generic_worker.js.
// This must be included in the project for Firefox to work,
// since the hack for obtaining the shim url does not work there.
// That is, hd_wasm::thread::script_path.js fails.

// Synchronously, using the browser, import out shim JS scripts
importScripts('pkg/hd_wasm.js');

// Wait for the main thread to send us the shared module/memory. Once we've got
// it, initialize it all with the `wasm_bindgen` global we imported via
// `importScripts`.
//
// After our first message all subsequent messages are an entry point to run,
// so we just do that.
self.onmessage = event => {
  let [module, memory] = event.data;
  let initialised = wasm_bindgen(module, memory).catch(err => {
    // Propagate to main `onerror`:
    setTimeout(() => {
      throw err;
    });
    // Rethrow to keep promise rejected and prevent execution of further commands:
    throw err;
  });

  self.onmessage = async event => {
    // This will queue further commands up until the module is fully initialised:
    await initialised;
    wasm_bindgen.generic_worker_entry_point(event.data);
  };
};
