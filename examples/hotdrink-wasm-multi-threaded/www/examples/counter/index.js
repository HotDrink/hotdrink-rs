// Instantiate the WebAssembly module, then run `main`.
wasm_bindgen('../../pkg/hotdrink_wasm_multi_threaded_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  const wasm = wasm_bindgen;
  const wrapper = wasm.CounterValueWrapper;

  let cs = wasm.counter_cs();
  cs.listen(e => cs.notify(e.data));

  // Update cs on button click
  let button = document.getElementById("button");
  button.addEventListener("click", () => cs.update());

  // Bind count
  let count = document.getElementById("count");
  count.addEventListener("input", () => {
    cs.set_variable("Counter", "count", wrapper.i32(parseInt(count.value)));
  })
  cs.subscribe("Counter", "count", new_count => count.value = new_count);
}
