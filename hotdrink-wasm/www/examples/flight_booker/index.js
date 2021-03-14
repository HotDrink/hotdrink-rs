// Instantiate the WebAssembly module, then run `main`.
wasm_bindgen('../../pkg/hotdrink_wasm_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  const wasm = wasm_bindgen;
  let wrap = wasm.FlightBookerValueWrapper;

  let cs = wasm.flight_booker_cs();
  cs.listen(e => cs.notify(e.data));

  // Bind flight type
  let flightType = document.getElementById("flight_type");
  flightType.onchange = () => {
    let value = flightType.value === "one-way-flight" ? 1 : 2;
    cs.set_variable("FlightBooker", "flight_type", wrap.FlightType(value));
    cs.update();
  };
  console.log(flightType);

  function bindDate(fieldName) {
    let field = document.getElementById(fieldName);
    // Send values to constraint system
    field.addEventListener("input", () => {
      cs.set_variable("FlightBooker", fieldName, wrap.f64(field.valueAsDate));
      cs.update();
    });
    // Receive values from constraint system
    cs.subscribe("FlightBooker", fieldName,
      new_value => field.valueAsDate = new Date(new_value),
    );
  }

  bindDate("start_date");
  bindDate("return_date");
}
