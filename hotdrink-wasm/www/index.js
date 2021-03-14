const wasm = wasm_bindgen;

async function main() {
    await wasm_bindgen('../pkg/hotdrink_wasm_bg.wasm');
    console.log(wasm.add(2, 3));
}

main();