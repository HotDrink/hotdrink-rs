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

License: MIT
