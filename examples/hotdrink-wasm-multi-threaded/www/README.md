# HotDrink WebAssembly

HotDrink implemented using Rust and WebAssembly.

Begin by downloading the prerequisites below.

* Rust (nightly)
* `wasm-pack`

Rust nightly can be used with `rustup default nightly`, and is needed for the experimental benchmarking features.

## Build

Build the project and compile Rust to WebAssembly using

```
wasm-pack build
```

You can then import the WebAssembly code from `pkg`, as shown in `example`.
