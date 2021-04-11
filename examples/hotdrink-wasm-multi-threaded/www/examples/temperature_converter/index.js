// Instantiate the WebAssembly module, then run `main`.
wasm_bindgen('../../pkg/hotdrink_wasm_multi_threaded_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  const wasm = wasm_bindgen;
  let wrap = wasm.TemperatureConverterValueWrapper;

  let cs = wasm.temperature_converter_cs();
  cs.listen(e => cs.notify(e.data));
  cs.update();

  function bindField(fieldComponent, fieldName) {
    let field = document.getElementById(fieldName);
    // Send values to constraint system
    field.addEventListener("input", () => {
      cs.set_variable(fieldComponent, fieldName, wrap.f64(parseFloat(field.value)));
      cs.update();
    });
    // Receive values from constraint system
    cs.subscribe(fieldComponent, fieldName,
      new_value => field.value = new_value
    );
  }

  bindField("TemperatureConverter", "kelvin");
  bindField("TemperatureConverter", "celsius");
  bindField("TemperatureConverter", "fahrenheit");
}
