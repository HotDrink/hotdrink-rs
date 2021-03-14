const wasm = wasm_bindgen;
wasm_bindgen('../../pkg/hotdrink_wasm_bg.wasm')
  .then(main)
  .catch(console.error);

function main() {
  let cs = wasm.js_cs_empty();

  let sum = 0;
  let count = 0;
  for (; count < 10; count++) {
    cs.touch_all();
    let before = performance.now();
    cs.update();
    let after = performance.now();
    sum += after - before;
  }

  console.log(`Solved cs ${count} times, average time ${sum / count} ms`);
}