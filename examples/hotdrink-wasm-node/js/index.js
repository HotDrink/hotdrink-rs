const hd = require("../pkg");
const i32 = hd.NumberWrapper.i32;

let cs = hd.sum();

function subscribe(variable) {
    cs.subscribe("Sum", variable, v => {
        console.log(`Event: ${variable} = ${v}`);
    });
}

console.log("Subscribing");
subscribe("a");
subscribe("b");
subscribe("c");

console.log("Setting a to 3");
cs.set_variable("Sum", "a", i32(3));
console.log("Updating");
cs.update();

console.log("Undoing last change");
cs.undo();

console.log("Redoing last change");
cs.redo();