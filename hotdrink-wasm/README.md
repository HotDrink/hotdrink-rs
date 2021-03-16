# hotdrink-wasm

A wrapper library around `hotdrink-rs` for compilation to WebAssembly.

## Prerequisites

Begin by downloading the prerequisites below.

* Rust (nightly)
* `wasm-pack`

To use Rust nightly for this project, either set it globally with `rustup default nightly`, or for this project with `rustup override set nightly`.
It is required for the experimental benchmarking features.

## Build

The standard library must be recompiled to use Web Workers from Rust, and the WebAssembly must be compiled with `--target no-modules` to be imported by them.
To run the example in `www`, use the makefile to perform the steps above, then read the instructions in that directory.


A wrapper library around `hotdrink-rs` for compilation to WebAssembly.

## Prerequisites

The project uses multiple nightly features, and must be built using nightly Rust.
I recommend using `rustup`, which can be downloaded [here](https://rustup.rs/),

You also need `wasm-pack`, which can be downloaded [here](https://rustwasm.github.io/wasm-pack/installer/).

## Build

The standard library must be recompiled to use Web Workers from Rust, and the WebAssembly must be compiled with `--target no-modules` to be imported by them.
To run the example in `www`, use the makefile to perform the steps above, then read the instructions in that directory.
If an appropriate version of Rust is installed, it should be as simple as running the following:

```bash
wasm-pack build --out-dir www/pkg --target no-modules --release
```

License: MIT
