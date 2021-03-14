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

function bench_component(name, make_component, n_variables) {
    var pm = new hd.PropertyModel();
    pm.add(make_component(n_variables));

    let total = 0;
    let n_samples = 1;
    for (let i = 0; i < n_samples; i++) {
        let start = performance.now();
        pm.update();
        total += performance.now() - start;
    }

    console.log(name, "&", n_variables, "&", total / n_samples);
}

function bench_component_update_variable(name, make_component, n_variables) {
    var pm = new hd.PropertyModel();
    pm.add(make_component(n_variables));

    let total = 0;
    let n_samples = 5;
    for (let i = 0; i < n_samples; i++) {
        let random_number = Math.floor(Math.random() * n_variables);
        for (let [k, v] of Object.entries(pm.variables)) {
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
    for (let n_variables of [5000]) {
        for (let entry of entries) {
            let name = entry.name;
            let make_component = entry.make_component;
            // bench_component(name, make_component, n_variables);
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
]);
