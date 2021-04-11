// Instantiate the WebAssembly module, then run `main`.
wasm_bindgen('../../pkg/hotdrink_wasm_multi_threaded_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  const wasm = wasm_bindgen;
  for (let i = 0; i < 5; i++) {
    wasm.bench_web_worker_init();
  }
}
