const wasm = require("../pkg/hd_wasm")

let component = wasm.demo_component();
component.update("a", 3);
console.log(component);