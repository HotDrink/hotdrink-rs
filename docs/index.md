Before getting started with the library, it will likely help to know what constraint systems do, and if they will help you.
While the original intent is for them to be used in GUIs, there is nothing stopping you from using it in any other program.

- If you are just using Rust, then take a look at [`hotdrink-rs`](./hotdrink-rs/hotdrink-rs.md), and use it as any other Rust-dependency.
  This will be easier to work with than the WebAssembly wrapper.
- If you want to use the library on the web, take a look at [`hotdrink-wasm`](hotdrink-wasm/hotdrink-wasm.md), which provides wrappers for defining constraint systems that can be used from JavaScript (or anything else that is able to interact with WebAssembly).