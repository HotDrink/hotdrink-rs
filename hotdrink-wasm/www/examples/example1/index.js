// Instantiate the WebAssembly module, then run `main`.
wasm_bindgen('../../pkg/hotdrink_wasm_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  const wasm = wasm_bindgen;
  const wrapper = wasm.MyValueWrapper;

  let cs = wasm.demo_cs();
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
      let before = performance.now();
      cs.update();
      let after = performance.now();
      console.log("Solve took", after - before, "ms");
    });
    // Subscribe to a variable in the given component
    cs.subscribe(comp, name, v => {
      box.value = v;
      if (state) {
        state.textContent = "";
        state.title = "";
      }
    }, () => {
      if (state) {
        state.textContent = "⌛";
        state.title = "";
      }
    }, e => {
      if (state) {
        state.textContent = "⛔";
        state.title = e;
      }
    }
    );
  }

  // When the checkbox is checked, pin its associated variable.
  // When it is unchecked, unpin it.
  function bindCheckbox(comp, name) {
    let pin = document.getElementById("pin_" + name);
    pin.addEventListener("input", () => {
      if (pin.checked) {
        cs.pin(comp, name);
      } else {
        cs.unpin(comp, name);
      }
    })
  }

  function bindNumber(comp, name) {
    return bindElement(comp, name, s => wrapper.f64(parseInt(s)));
  }

  function bindText(comp, name) {
    return bindElement(comp, name, s => wrapper.String(s));
  }

  function bindCircle(comp, name) {
    // Get the circle fields
    let x = document.getElementById(name + "_x");
    let y = document.getElementById(name + "_y");
    let r = document.getElementById(name + "_r");
    // Create a listener that reads the fields and sends a new circle to the constraint system
    let listener = () => {
      let circle = new wasm.Circle(
        parseInt(x.value),
        parseInt(y.value),
        parseInt(r.value)
      );
      cs.set_variable(comp, name, wrapper.Circle(circle));
      cs.update();
    };
    // Add the listener to the fields
    x.addEventListener("input", listener);
    y.addEventListener("input", listener);
    r.addEventListener("input", listener);
    // Add a callback to the constraint system to get notifications when a new value has been computed
    cs.subscribe(comp, name, new_circle => {
      x.value = new_circle.x;
      y.value = new_circle.y;
      r.value = new_circle.r;
    }, () => { });
  }

  let arithmetic = "Arithmetic";
  bindNumber(arithmetic, "a");
  bindNumber(arithmetic, "b");
  bindNumber(arithmetic, "c");
  bindNumber(arithmetic, "d");
  bindCheckbox(arithmetic, "a");
  bindCheckbox(arithmetic, "b");
  bindCheckbox(arithmetic, "c");
  bindCheckbox(arithmetic, "d");

  let concat = "concat";
  bindText(concat, "e");
  bindText(concat, "f");
  bindText(concat, "g");

  let fib = "fib";
  bindNumber(fib, "fib_in_slider");
  bindNumber(fib, "fib_in");
  bindNumber(fib, "fib_out");

  let circle = "circle";
  bindCircle(circle, "circle_a");
  bindCircle(circle, "circle_b");

  let db = "db";
  bindText(db, "db_name");
  bindText(db, "db_age");
  bindText(db, "db_double_age");

  let fib_chain = "fibs";
  bindNumber(fib_chain, "chain1");
  bindNumber(fib_chain, "chain2");
  bindNumber(fib_chain, "chain3");
  bindNumber(fib_chain, "chain4");
  bindNumber(fib_chain, "chain4p2");
  bindNumber(fib_chain, "chain5");

  cs.update();
}
