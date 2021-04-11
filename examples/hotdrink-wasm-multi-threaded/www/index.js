const wasm = wasm_bindgen;

async function main() {
    await wasm_bindgen('../pkg/hotdrink_wasm_multi_threaded_bg.wasm');
    // `wasm` can now be used to access everything marked with `wasm_bindgen`.
    console.log("Loaded:", wasm);
}

main();