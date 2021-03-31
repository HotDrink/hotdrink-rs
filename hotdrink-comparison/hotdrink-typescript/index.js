function id(x) {
    return x;
}

function vname(i) {
    return "v" + i;
}

function make_empty(n_variables) {
    var model = new hd.ComponentBuilder();
    for (let i = 0; i < n_variables; i++) {
        model.variable(vname(i), 0);
    }
    return model.component();
}

function make_dense(n_variables) {
    var model = new hd.ComponentBuilder();
    for (let i = 0; i < n_variables; i++) {
        model.variable(vname(i), 0);
    }
    for (let i = 0; i < n_variables - 4; i += 4) {
        for (let offset = 1; offset <= 4; offset++) {
            let from = vname(i);
            let to = vname(i + offset);

            model.constraint(`${from}, ${to}`);
            model.method(`${from} -> ${to}`, id);
            model.method(`${to} -> ${from}`, id);
        }
    }
    return model.component();
}

function make_linear_twoway(n_variables) {
    var model = new hd.ComponentBuilder();
    for (let i = 0; i < n_variables; i++) {
        model.variable(vname(i), 0);
    }
    for (let i = 0; i < n_variables - 1; i++) {
        let a = vname(i);
        let b = vname(i + 1);
        model.constraint(`${a}, ${b}`)
        model.method(`${a} -> ${b}`, id);
        model.method(`${b} -> ${a}`, id);
    }
    return model.component();
}

function make_linear_oneway(n_variables) {
    var model = new hd.ComponentBuilder();
    for (let i = 0; i < n_variables; i++) {
        model.variable(vname(i), 0);
    }
    for (let i = 0; i < n_variables - 1; i++) {
        let a = vname(i);
        let b = vname(i + 1);
        model.constraint(`${a}, ${b}`)
        model.method(`${a} -> ${b}`, id);
    }
    return model.component();
}

function make_unprunable(n_variables) {
    var model = new hd.ComponentBuilder();
    for (let i = 0; i < n_variables; i++) {
        model.variable(vname(i), 0);
    }
    for (let i = 0; 2 * i + 2 < n_variables - 1; i++) {
        let current = vname(i);
        let left = vname(2 * i + 1);
        let right = vname(2 * i + 2);
        model.constraint(`${current}, ${left}, ${right}`)
        model.method(`${current}, ${left} -> ${right}`, (a, b) => a + b);
        model.method(`${current}, ${right} -> ${left}`, (a, b) => a + b);
    }
    return model.component();
}

function make_ladder(n_variables) {
    var model = new hd.ComponentBuilder();
    for (let i = 0; i < n_variables; i++) {
        model.variable(vname(i), 0);
    }
    for (let i = 0; i < n_variables - 3; i += 2) {
        let a0 = vname(i);
        let b0 = vname(i + 1);
        let a1 = vname(i + 2);
        let b1 = vname(i + 3);

        model.constraint(`${a0}, ${b0}, ${a1}`);
        model.method(`${a0}, ${b0} -> ${a1}`, (a, b) => 0);
        model.method(`${a0}, ${a1} -> ${b0}`, (a, b) => 0);
        model.method(`${b0}, ${a1} -> ${a0}`, (a, b) => 0);

        model.constraint(`${b1}, ${b0}, ${a1}`);
        model.method(`${b1}, ${b0} -> ${a1}`, (a, b) => 0);
        model.method(`${b1}, ${a1} -> ${b0}`, (a, b) => 0);
        model.method(`${b0}, ${a1} -> ${b1}`, (a, b) => 0);
    }
    return model.component();
}

function random(min, max) {
    return Math.floor(Math.random() * (max - min) + min);
}

function swap_remove(array, index) {
    if (index === array.length - 1) {
        return array.pop();
    }
    let last = array.pop();
    let to_remove = array[index];
    array[index] = last;
    return to_remove;
}

function shuffleArray(array) {
    for (let i = array.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [array[i], array[j]] = [array[j], array[i]];
    }
}

function make_random(n_constraints) {
    var model = new hd.ComponentBuilder();
    let current_constraints = 0;
    let variables_per_constraint = 5;

    // Start with n_constraints + 1, as each constraint should use at least 1 unused variable
    let n_variables = n_constraints + 1;
    let variables = [];
    for (let i = 0; i < n_variables; i++) {
        variables.push(i);
        model.variable(vname(i), 0);
    }

    // Add all current variables
    let used = [0];
    let unused = variables.slice(1);
    while (current_constraints < n_constraints) {
        // If there are no more unused variables, add a new one
        if (unused.length == 0) {
            variables.push(n_variables);
            unused.push(n_variables);
            model.variable(vname(n_variables), 0);
            n_variables++;
        }

        // Pick a random used variable
        let used_idx = random(0, used.length);
        let used_val = used[used_idx];

        // Pick a random unused variable
        let unused_idx = random(0, unused.length);
        let unused_val = swap_remove(unused, unused_idx);
        // Add it to used
        used.push(unused_val);

        // Add remaining variables, can be used or unused
        let buffer = [];
        for (let i = 0; i < variables_per_constraint - 2; i++) {
            let idx = random(0, variables.length);
            // Remove temporarily to avoid duplicates
            let val = swap_remove(variables, idx);
            buffer.push(val);
        }
        // Add back to variables
        variables.push(...buffer);
        buffer = buffer.filter(x => x != used_val && x != unused_val);
        unused = unused.filter(x => !buffer.includes(x));

        // console.log("Buffer", buffer);
        // console.log("Used", used_val);
        // console.log("Unused", unused_val);
        // Add all variables to constraint
        let inputs_string = "";
        for (let v of buffer) {
            inputs_string += vname(v) + ", ";
        }
        model.constraint(inputs_string + vname(used_val) + ", " + vname(unused_val));

        // Add methods that write to used and unused
        model.method(inputs_string + vname(unused_val) + " -> " + vname(used_val), arr => 0);
        model.method(inputs_string + vname(used_val) + " -> " + vname(unused_val), arr => 0);

        // Add remaining methods
        let n_methods = random(0, 3);
        // console.log("Adding", n_methods, " methods");
        // console.log(buffer);
        for (let i = 0; i < Math.min(n_methods, buffer.length); i++) {
            let output = buffer[i];
            // console.log("Output", output);
            let inputs = [];
            for (let j = 0; j < buffer.length; j++) {
                // Ignore the output
                if (i != j) {
                    inputs.push(buffer[j]);
                }
            }
            // console.log("Inputs", inputs);
            let inputs_string = "";
            for (let j = 0; j < inputs.length; j++) {
                inputs_string += vname(inputs[j]) + ", ";
            }
            let mstr = inputs_string + vname(used_val) + ", " + vname(unused_val) + " -> " + vname(output);
            // console.log("mstr:", mstr);
            model.method(mstr, arr => 0);
        }

        current_constraints++;
    }

    return model.component();
}

function bench_component(name, make_component, n_variables) {
    let total = 0;
    let n_samples = 5;
    for (let i = 0; i < n_samples; i++) {
        // Generate a new component (for random)
        var pm = new hd.PropertyModel();
        pm.add(make_component(n_variables));

        let start = performance.now();
        pm.update();
        total += performance.now() - start;
    }

    console.log(name, "&", n_variables, "&", total / n_samples);
}

function bench_component_update_variable(name, make_component, n_variables) {

    let total = 0;
    let n_samples = 5;
    for (let i = 0; i < n_samples; i++) {
        // Generate a new component (for random)
        var pm = new hd.PropertyModel();
        pm.add(make_component(n_variables));

        let entries = Object.entries(pm.variables);
        let random_number = Math.floor(Math.random() * entries.length);
        for (let [k, v] of entries) {
            if (k.startsWith(vname(random_number))) {
                pm.variableChanged(v);
            }
        }
        let start = performance.now();
        pm.update();
        total += performance.now() - start;
    }

    console.log(name, "&", n_variables, "&", total / n_samples);
}

function bench_components(entries) {
    for (let n_variables of [1250, 2500, 5000]) {
        for (let entry of entries) {
            let name = entry.name;
            let make_component = entry.make_component;
            bench_component(name, make_component, n_variables);
        }
    }
}

function bench_components_update_variable(entries) {
    for (let n_variables of [1250, 2500, 5000]) {
        for (let entry of entries) {
            let name = entry.name;
            let make_component = entry.make_component;
            bench_component_update_variable(name, make_component, n_variables);
        }
    }
}

bench_components([
    { name: "empty", make_component: make_empty },
    { name: "dense", make_component: make_dense },
    { name: "linear-oneway", make_component: make_linear_oneway },
    { name: "linear-twoway", make_component: make_linear_twoway },
    { name: "ladder", make_component: make_ladder },
    { name: "unprunable", make_component: make_unprunable },
    { name: "random", make_component: make_random },
]);

bench_components_update_variable([
    { name: "empty", make_component: make_empty },
    { name: "dense", make_component: make_dense },
    { name: "linear-oneway", make_component: make_linear_oneway },
    { name: "linear-twoway", make_component: make_linear_twoway },
    { name: "ladder", make_component: make_ladder },
    { name: "unprunable", make_component: make_unprunable },
    { name: "random", make_component: make_random },
]);
