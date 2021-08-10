// Instantiate the WebAssembly module, then run `main`.
wasm_bindgen('../../pkg/hotdrink_wasm_multi_threaded_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  const wasm = wasm_bindgen;
  const wrapper = wasm.StringWrapper;

  let cs = wasm.text_search();
  cs.listen(e => cs.notify(e.data));

  // Bind a single HTML element to the constraint system.
  // Get the field named `name`, and upon edits send it to the constraint system and solve.
  // Add a callback to the constraint system to set the field value when a new value arrives.
  function bindElement(comp, name, parse) {
    let box = document.getElementById(name);
    let state = document.getElementById(name + "_state");
    // Pass input events to the constraint system
    box.addEventListener("input", () => {
      let parsed = parse(box.value);
      cs.set_variable(comp, name, parsed);
      cs.update();
    });
    // Subscribe to a variable in the given component
    cs.subscribe(comp, name,
      // On ready
      v => {
        box.value = v;
        box.classList.add("is-valid");
        box.classList.remove("is-invalid");
      },
      // On pending
      () => {
        box.classList.remove("is-valid");
        box.classList.remove("is-invalid");
      },
      // On error
      e => {
        box.classList.remove("is-valid");
        box.classList.add("is-invalid");
      }
    );
  }

  function bindNumber(comp, name) {
    return bindElement(comp, name, s => wrapper.i32(parseInt(s)));
  }

  function bindText(comp, name) {
    return bindElement(comp, name, s => wrapper.String(s));
  }

  let c = "Component";
  bindText(c, "input");
  bindText(c, "query");
  bindText(c, "output");

  cs.update();
}
