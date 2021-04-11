function make_component(name, n_variables, add_constraint) {
  let cmp = new hd.Component(name);

  let variables = [];
  for (let i = 0; i < n_variables; i++) {
    let v = cmp.emplaceVariable(i.toString(), 0);
    variables.push(v);
  }

  for (let i = 0; i < n_variables - 1; i++) {
    add_constraint(cmp, variables, i);
  }

  return cmp;
}

function make_linear_oneway(n_constraints) {
  let n_variables = n_constraints + 1;
  return make_component("linear-oneway", n_variables, (cmp, variables, i) => {
    let m1 = new hd.Method(2, [0], [1], [hd.maskNone], (a) => { return a; }, "m1");
    cmp.emplaceConstraint(i.toString(), new hd.ConstraintSpec([m1]), [variables[i], variables[i + 1]]);
  });
}

function make_linear_twoway(n_constraints) {
  let n_variables = n_constraints + 1;
  return make_component("linear-twoway", n_variables, (cmp, variables, i) => {
    let m1 = new hd.Method(2, [0], [1], [hd.maskNone], (a) => { return a; }, "m1");
    let m2 = new hd.Method(2, [1], [0], [hd.maskNone], (a) => { return a; }, "m2");
    cmp.emplaceConstraint(i.toString(), new hd.ConstraintSpec([m1, m2]), [variables[i], variables[i + 1]]);
  });
}

function avg(x, y) { return (x + y) / 2; }
function rev(avg, x) { return 2 * avg - x; }
function id(x) { return x; }

function make_ladder(n_constraints) {
  let n_variables = n_constraints + 2;
  return make_component("ladder", n_variables, (cmp, variables, i) => {
    if (i <= 3 || i + 3 >= variables.length) { return; }
    if (i % 2 != 0) { return; }

    let a0 = i;
    let b0 = i + 1;
    let a1 = i + 2;
    let b1 = i + 3;

    let m1 = new hd.Method(3, [0, 1], [2], [hd.maskNone, hd.maskNone], id, "m1");
    let m2 = new hd.Method(3, [0, 2], [1], [hd.maskNone, hd.maskNone], id, "m2");
    let m3 = new hd.Method(3, [1, 2], [0], [hd.maskNone, hd.maskNone], id, "m3");
    cmp.emplaceConstraint(i.toString(), new hd.ConstraintSpec([m1, m2, m3]), [variables[a0], variables[b0], variables[a1]]);

    let m4 = new hd.Method(3, [0, 1], [2], [hd.maskNone, hd.maskNone], id, "m4");
    let m5 = new hd.Method(3, [0, 2], [1], [hd.maskNone, hd.maskNone], id, "m5");
    let m6 = new hd.Method(3, [1, 2], [0], [hd.maskNone, hd.maskNone], id, "m6");
    cmp.emplaceConstraint((i + 1).toString(), new hd.ConstraintSpec([m4, m5, m6]), [variables[b0], variables[a1], variables[b1]]);
  })
}

function make_unprunable(n_constraints) {
  let depth = Math.log2(n_constraints);
  let n_variables = Math.pow(2, depth + 1.0);
  return make_component("unprunable", n_variables, (cmp, variables, i) => {
    let leftIdx = 2 * i + 1;
    let rightIdx = 2 * i + 2;
    if (leftIdx >= n_variables || rightIdx >= n_variables) {
      return;
    }
    let m1 = new hd.Method(3, [0, 1], [2], [hd.maskNone], (a) => { return a; }, "m1");
    let m2 = new hd.Method(3, [0, 2], [1], [hd.maskNone], (a) => { return a; }, "m2");
    cmp.emplaceConstraint(i.toString(), new hd.ConstraintSpec([m1, m2]), [variables[i], variables[leftIdx], variables[rightIdx]]);
  });
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
  var model = new hd.Component("random");
  let current_constraints = 0;
  let variables_per_constraint = 5;

  // Start with n_constraints + 1, as each constraint should use at least 1 unused variable
  let n_variables = n_constraints + 1;
  let variable_indices = [];
  let variables = [];
  for (let i = 0; i < n_variables; i++) {
    variable_indices.push(i);
    let v = model.emplaceVariable("v" + i.toString(), 0);
    variables.push(v);
  }

  // Add all current variables
  let used = [0];
  let unused = variable_indices.slice(1);
  while (current_constraints < n_constraints) {
    // If there are no more unused variables, add a new one
    if (unused.length == 0) {
      variable_indices.push(n_variables);
      unused.push(n_variables);
      let v = model.emplaceVariable("v" + n_variables.toString(), 0);
      variables.push(v);
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
      let idx = random(0, variable_indices.length);
      // Remove temporarily to avoid duplicates
      let val = swap_remove(variable_indices, idx);
      buffer.push(val);
    }
    // Add back to variables
    variable_indices.push(...buffer);
    buffer = buffer.filter(x => x != used_val && x != unused_val);
    unused = unused.filter(x => !buffer.includes(x));

    // console.log("Buffer", buffer);
    // console.log("Used", used_val);
    // console.log("Unused", unused_val);

    // Add methods that write to used and unused
    let bufIndices = [];
    for (let i = 0; i < buffer.length; i++) {
      bufIndices.push(i);
    }
    let m_used_idx = buffer.length;
    let m_unused_idx = buffer.length + 1;
    let methods = [
      new hd.Method(buffer.length + 2, [...bufIndices, m_unused_idx], [m_used_idx], Array(buffer.length + 1).fill(hd.maskNone), "write_used"),
      new hd.Method(buffer.length + 2, [...bufIndices, m_used_idx], [m_unused_idx], Array(buffer.length + 1).fill(hd.maskNone), "write_unused"),
    ];

    // Add remaining methods
    let n_methods = random(0, 3);
    // console.log("Adding", n_methods, " methods");
    // console.log(buffer);
    for (let i = 0; i < Math.min(n_methods, buffer.length); i++) {
      let output = i;
      // console.log("Output", output);
      let inputs = [];
      for (let j = 0; j < buffer.length; j++) {
        // Ignore the output
        if (i != j) {
          inputs.push(j);
        }
      }
      // console.log("Inputs", inputs);
      // console.log("mstr:", mstr);
      methods.push(new hd.Method(buffer.length + 2, inputs.concat([m_used_idx, m_unused_idx]), [output], Array(inputs.length + 2).fill(hd.maskNone), "write_" + buffer[output]));
    }

    let allVarIndices = buffer.concat([used_val, unused_val]);
    let allVars = allVarIndices.map(i => variables[i]);
    model.emplaceConstraint("c" + current_constraints.toString(), new hd.ConstraintSpec(methods), allVars)

    current_constraints++;
  }

  return model;
}

function bench_component(name, make_component, n_variables) {
  let cmp = make_component(n_variables);
  let cs = new hd.ConstraintSystem();
  cmp.connectSystem(cs);

  let total = 0;
  let n_samples = 5;
  for (let i = 0; i < n_samples; i++) {
    // Collect variables
    let variables = []
    for (v of cmp._vars) {
      variables.push(v);
    }
    // Get random index
    let random_index = Math.floor(Math.random() * n_variables);
    let random_variable = variables[random_index];

    // Perform set/update
    let start = performance.now();
    random_variable.set(0);
    cs.update();
    total += performance.now() - start;
  }

  console.log(name, "&", n_variables, "&", total / n_samples);
}

function bench_component_max(name, make_component, n_variables) {
  let cmp = make_component(n_variables);
  let cs = new hd.ConstraintSystem();
  cmp.connectSystem(cs);

  let max = 0;
  let n_samples = 5;
  for (let i = 0; i < n_samples; i++) {
    // Collect variables
    let variables = []
    for (v of cmp._vars) {
      variables.push(v);
    }
    // Get random index
    let random_index = Math.floor(Math.random() * n_variables);
    let random_variable = variables[random_index];

    // Perform set/update
    let start = performance.now();
    random_variable.set(0);
    cs.update();
    max = Math.max(max, performance.now() - start);
  }

  console.log(name, "&", n_variables, "&", max);
}

function bench_components(entries) {
  console.log("------- Average -------")
  for (let n_variables of [100, 500, 1000]) {
    for (let entry of entries) {
      bench_component(entry.name, entry.make_component, n_variables);
    }
  }
  console.log("------- Max -------")
  for (let n_variables of [100, 500, 1000]) {
    for (let entry of entries) {
      bench_component_max(entry.name, entry.make_component, n_variables);
    }
  }
}

bench_components([
  { name: "linear-oneway", make_component: make_linear_oneway },
  { name: "linear-twoway", make_component: make_linear_twoway },
  { name: "ladder       ", make_component: make_ladder },
  { name: "random       ", make_component: make_random },
  { name: "unprunable   ", make_component: make_unprunable }
])
