// Instantiate the WebAssembly module, then run `main`.
wasm_bindgen('../../pkg/hotdrink_wasm_multi_threaded_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  const wasm = wasm_bindgen;
  const wrapper = wasm.NumberWrapper;

  let cs = wasm.parallel();
  cs.listen(e => cs.notify(e.data));

  // Capture undo events
  document.addEventListener('keydown', (event) => {
    if (event.ctrlKey && event.key === 'z') {
      event.preventDefault();
      cs.undo();
    }
    if (event.ctrlKey && event.key === 'Z') {
      event.preventDefault();
      cs.redo();
    }
  });

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
        box.classList.add("is-valid");
        box.classList.remove("is-invalid");
        box.value = v;
        if (state) {
          state.textContent = "";
          state.title = "";
        }
      },
      // On pending
      () => {
        box.classList.remove("is-valid");
        box.classList.remove("is-invalid");
        if (state) {
          state.textContent = "⌛";
          state.title = "";
        }
      },
      // On error
      e => {
        box.classList.remove("is-valid");
        box.classList.add("is-invalid");
        if (state) {
          state.textContent = "⛔";
          state.title = e;
        }
      }
    );
  }

  function bindNumber(comp, name) {
    return bindElement(comp, name, s => wrapper.i32(parseInt(s)));
  }

  function bindText(comp, name) {
    return bindElement(comp, name, s => wrapper.String(s));
  }

  let a = "A";
  bindNumber(a, "a");
  bindNumber(a, "b");
  bindNumber(a, "c");
  bindNumber(a, "d");
  bindNumber(a, "e");
  bindNumber(a, "f");
  bindNumber(a, "g");
  bindNumber(a, "h");
  bindNumber(a, "i");

  cs.update();
}
