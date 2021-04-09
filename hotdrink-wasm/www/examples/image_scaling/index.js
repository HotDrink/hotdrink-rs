// Instantiate the WebAssembly module, then run `main`.
wasm_bindgen('../../pkg/hotdrink_wasm_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  const wasm = wasm_bindgen;
  const wrapper = wasm.NumberWrapper;

  let cs = wasm.image_scaling();
  cs.listen(e => cs.notify(e.data));
  cs.update();
  let component = "ImageScaling";

  function bind(variable, parse = s => wrapper.i32(parseInt(s))) {
    let box = document.getElementById(variable);
    // Send events to the constraint system
    box.addEventListener("input", () => {
      cs.set_variable(component, variable, parse(box.value));
      cs.update();
    })
    // Receive events from the constraint system
    cs.subscribe(component, variable, v => {
      console.log(`${component}.${variable} = ${v}`);
      box.value = v;
    }, () => { }, e => console.error(e));
  }

  document.getElementById("initial_height").contentEditable = false;
  bind("initial_height");
  bind("initial_width");
  bind("absolute_height");
  bind("absolute_width");
  bind("relative_height");
  bind("relative_width");

  // Bind the checkbox to pinning the aspect raio
  let aspectRatio = document.getElementById("aspect_ratio_checkbox");
  aspectRatio.addEventListener("change", () => {
    if (aspectRatio.checked) {
      cs.pin(component, "aspect_ratio");
    } else {
      cs.unpin(component, "aspect_ratio");
    }
  });
}
