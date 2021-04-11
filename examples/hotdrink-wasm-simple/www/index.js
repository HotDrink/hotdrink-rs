import * as hd from "hotdrink-wasm-simple"

const wrapper = hd.NumberWrapper;

// Initialize constraint system
let cs = hd.image_scaling();
let component = "ImageScaling";

// Set initial width and height to image size
let image = document.getElementById("image");
cs.set_variable(component, "initial_width", wrapper.i32(image.width));
cs.set_variable(component, "initial_height", wrapper.i32(image.height));
cs.update();

// A function that connects a HTML element to a constraint system variable
function bind(variable) {
    let box = document.getElementById(variable);
    // Send events to the constraint system
    box.addEventListener("input", () => {
        cs.set_variable(component, variable, wrapper.i32(parseInt(box.value)));
        cs.update();
    })
    // Receive events from the constraint system
    cs.subscribe(component, variable,
        v => {
            // Update the field
            box.value = v;
            // Update the image
            let image = document.getElementById("image");
            image.width = parseInt(document.getElementById("absolute_width").value);
            image.height = parseInt(document.getElementById("absolute_height").value);
        },
        _ => { }, // Handle pending-events
        console.error // Handle error-events
    );
}

bind("initial_height");
bind("initial_width");

bind("absolute_height");
bind("absolute_height_range");

bind("absolute_width");
bind("absolute_width_range");

bind("relative_height");
bind("relative_width");

// Bind the checkbox to pinning the aspect raio
let aspectRatio = document.getElementById("aspect_ratio_checkbox");
aspectRatio.addEventListener("change", () => {
    if (aspectRatio.checked) {
        cs.pin(component, "aspect_ratio");
    } else {
        cs.unpin(component, "aspect_ratio");
    }
});