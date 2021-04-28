# hotdrink-rs

An implementation of HotDrink in Rust, with a wrapper library for compilation to WebAssembly.

Warning: This library is still highly experimental, and the API is likely to change.
While I would not recommend that this library is used in production, any feedback would be great.

## hotdrink-rs

The main part of the library, and can be used as-is as a normal Rust dependency.
Go here if you are writing an application in Rust.

## hotdrink-wasm

A wrapper around `hotdrink-rs` for compilation to WebAssembly.
Go here if you are writing an application in JavaScript, or any other language that can use WebAssembly.

## hotdrink-examples

Go here if you want to see full examples of how to use the `hotdrink-rs` with a Rust GUI library,
or `hotdrink-wasm` with various frameworks (both with and without multithreading).

## benches

Benchmarks of other implementations of HotDrink to compare to.