# Examples

Examples of how to use `hotdrink-wasm`.

* `hotdrink-rs-iced` shows how to use `hotdrink-rs` with `iced`, a GUI library in Rust.
* `hotdrink-wasm-simple` is a simple example of how to use single threaded `hotdrink-wasm` in a JavaScript project with `npm` and `webpack`.
  Use this if you just want to get started with `hotdrink-wasm`.
* `hotdrink-wasm-multi-threaded` is an example of how to use multi-threaded `hotdrink-wasm` in a JavaScript project with `npm` and `webpack`.
  This is more difficult to set up, but is required if you don't want long-running computations in your methods to block the GUI.
  All methods will be computed outside of the main thread, keeping it responsive at all times.
* `hotdrink-wasm-node` is an example of how to use `hotdrink-wasm` with `Node.js`.
* `hotdrink-wasm-react` is an example of how to use `hotdrink-wasm` with `React`.
  