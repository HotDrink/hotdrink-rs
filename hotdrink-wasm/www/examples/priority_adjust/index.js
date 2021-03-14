// Instantiate the WebAssembly module, then run `main`.
wasm_bindgen('../../pkg/hotdrink_wasm_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  const wasm = wasm_bindgen;
  const wrapper = wasm.StringWrapper;

  let cs = wasm.priority_adjust();
  cs.listen(e => cs.notify(e.data));

  function bindElement(comp, name, parse) {
    let box = document.getElementById(name);
    // Pass input events to the constraint system
    box.addEventListener("input", () => {
      let parsed = parse(box.value);
      cs.set_variable(comp, name, parsed);
      cs.update();
    });
    // Subscribe to a variable in the given component
    cs.subscribe(comp, name, v => {
      box.value = v;
    });
  }

  function bindNumber(comp, name) {
    bindElement(comp, name, s => wrapper.String(s));
  }

  let component = "PriorityAdjust";
  bindNumber(component, "a");
  bindNumber(component, "b");
  bindNumber(component, "c");
  bindNumber(component, "d");
}
