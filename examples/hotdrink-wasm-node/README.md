# `hotdrink-wasm-node`

An example of how to use `hotdrink-wasm` with `Node.js`.

First, compile with `wasm-pack` with the `nodejs` target.

```
wasm-pack build --release --target nodejs
```

Then you can run your JavaScript code with `node`.

```
node ./js/index.js
```