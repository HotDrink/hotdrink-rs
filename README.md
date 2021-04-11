# hotdrink-rs

An implementation of HotDrink in Rust, with a wrapper library for compilation to WebAssembly.

Warning: This library is still highly experimental, and the API is likely to change.
While I would not recommend that this library is used in production, any feedback would be great.

## hotdrink-rs

The main part of the library, and can be used as-is as a normal Rust dependency.
Go here if you are writing an application in Rust.

## hotdrink-rs-example

An example of how a Rust GUI application can use `hotdrink-rs`.

## hotdrink-wasm

A wrapper around `hotdrink-rs` for compilation to WebAssembly.
Go here if you are writing an application in JavaScript, or any other language that can use WebAssembly.

## hotdrink-wasm-example (TODO)

Go here if you want an example of how to use the library from JavaScript.

## hotdrink-comparison

Benchmarks of other implementations of HotDrink to compare to.