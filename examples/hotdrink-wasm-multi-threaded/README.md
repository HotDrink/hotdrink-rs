# Examples for hotdrink-wasm with multithreading

Multi-threaded examples for `hotdrink-wasm`.

```
make # Compile the library to WebAssembly (output in `pkg`)
cd www # Go to the web-application directory.
npm install # Install dependencies
npm start # Run the examples locally (localhost:8080)
```

Note that some extra flags must be set for threads to work, such as `--target no-modules`.
The standard library must also be recompiled with support for atomics.
The makefile together with `.cargo/config.toml` will do this configuration for you.

See https://rustwasm.github.io/docs/wasm-bindgen/examples/raytrace.html for more information.