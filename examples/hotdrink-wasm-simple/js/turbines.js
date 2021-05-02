import("../pkg").then(hd => {
    const wrapper = hd.NumberWrapper;

    // Initialize constraint system
    let cs = hd.turbines();
    let component = "Turbines";
    cs.update();

    // Auto wind
    let avg = 0;
    let weight = 0.99;
    setInterval(() => {
        avg = weight * avg + (1 - weight) * (Math.random() * 100);
        console.log("Setting speed to", avg);
        document.getElementById("wind_speed").value = avg.toFixed(0);
        cs.set_variable(component, "wind_speed", wrapper.f64(avg));
        cs.update();
    }, 100);


    // A function that connects a HTML element to a constraint system variable
    function bind(variable) {
        let box = document.getElementById(variable);
        // Send events to the constraint system
        box.addEventListener("input", () => {
            cs.set_variable(component, variable, wrapper.f64(parseFloat(box.value)));
            cs.update();
        })
        // Receive events from the constraint system
        cs.subscribe(component, variable,
            v => box.value = v.toFixed(0),
            _ => { }, // Handle pending-events
            console.error // Handle error-events
        );
    }

    bind("blade_length");
    bind("air_density");
    bind("efficiency");
    bind("wind_speed");
    bind("wind_power");
    bind("power_output");
})