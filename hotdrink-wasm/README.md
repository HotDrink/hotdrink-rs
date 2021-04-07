# hotdrink-wasm

A wrapper library around `hotdrink-rs` for compilation to WebAssembly.

## Prerequisites

The project uses multiple nightly features, and must be built using nightly Rust.
I recommend using `rustup`, which can be downloaded [here](https://rustup.rs/),

You also need `wasm-pack`, which can be downloaded [here](https://rustwasm.github.io/wasm-pack/installer/).

The standard library must be recompiled, which means that we need the standard library source code.
This can be downloaded with `rustup component add rust-src`.

## Build

To use Web Workers from Rust, the we must compile with `--target no-modules`.
This should be as simple as running the following:

```bash
wasm-pack build --out-dir www/pkg --target no-modules --release
```

This will produce WebAssembly code and JS wrappers in www/pkg, which can then be imported there.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
