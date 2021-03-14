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

function make_empty(n_variables) {
  return make_component("empty", n_variables, (cmp, variables, i) => { });
}

function make_linear_oneway(n_variables) {
  return make_component("linear-oneway", n_variables, (cmp, variables, i) => {
    let m1 = new hd.Method(2, [0], [1], [hd.maskNone], (a) => { return a; }, "m1");
    cmp.emplaceConstraint(i.toString(), new hd.ConstraintSpec([m1]), [variables[i], variables[i + 1]]);
  });
}

function make_linear_twoway(n_variables) {
  return make_component("linear-twoway", n_variables, (cmp, variables, i) => {
    let m1 = new hd.Method(2, [0], [1], [hd.maskNone], (a) => { return a; }, "m1");
    let m2 = new hd.Method(2, [1], [0], [hd.maskNone], (a) => { return a; }, "m2");
    cmp.emplaceConstraint(i.toString(), new hd.ConstraintSpec([m1, m2]), [variables[i], variables[i + 1]]);
  });
}

function make_dense(n_variables) {
  return make_component("dense", n_variables, (cmp, variables, i) => {
    if (i % 4 != 0) return;
    for (let offset = 1; offset <= 4; offset++) {
      let from = i;
      let to = i + offset;
      if (to >= variables.length) return;

      let m1 = new hd.Method(2, [0], [1], [hd.maskNone], a => a, "m1");
      let m2 = new hd.Method(2, [1], [0], [hd.maskNone], a => a, "m2");
      cmp.emplaceConstraint(to.toString(), new hd.ConstraintSpec([m1, m2]), [variables[from], variables[to]]);
    }
  });
}

function make_sparse(n_variables) {
  return make_component("sparse", n_variables, (cmp, variables, i) => {
    if (i % 5 != 0) return;
    for (let offset = 1; offset <= 5; offset += 2) {
      let from = i;
      let to = i + offset;
      if (to >= variables.length) return;

      let m1 = new hd.Method(2, [0], [1], [hd.maskNone], a => a, "m1");
      let m2 = new hd.Method(2, [1], [0], [hd.maskNone], a => a, "m2");
      cmp.emplaceConstraint(to.toString(), new hd.ConstraintSpec([m1, m2]), [variables[from], variables[to]]);
    }
  });
}

function make_unprunable(n_variables) {
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

function avg(x, y) { return (x + y) / 2; }
function rev(avg, x) { return 2 * avg - x; }

function make_ladder(n_variables) {
  return make_component("ladder", n_variables, (cmp, variables, i) => {
    if (i <= 3 || i + 3 >= variables.length) { return; }
    if (i % 2 != 0) { return; }

    let a0 = i;
    let b0 = i + 1;
    let a1 = i + 2;
    let b1 = i + 3;

    let m1 = new hd.Method(3, [0, 1], [2], [hd.maskNone, hd.maskNone], avg, "m1");
    let m2 = new hd.Method(3, [0, 2], [1], [hd.maskNone, hd.maskNone], avg, "m2");
    let m3 = new hd.Method(3, [1, 2], [0], [hd.maskNone, hd.maskNone], avg, "m3");
    cmp.emplaceConstraint(i.toString(), new hd.ConstraintSpec([m1, m2, m3]), [variables[a0], variables[b0], variables[a1]]);

    let m4 = new hd.Method(3, [0, 1], [2], [hd.maskNone, hd.maskNone], avg, "m4");
    let m5 = new hd.Method(3, [0, 2], [1], [hd.maskNone, hd.maskNone], avg, "m5");
    let m6 = new hd.Method(3, [1, 2], [0], [hd.maskNone, hd.maskNone], avg, "m6");
    cmp.emplaceConstraint((i + 1).toString(), new hd.ConstraintSpec([m4, m5, m6]), [variables[b0], variables[a1], variables[b1]]);
  })
}

function bench_component(make_component, n_variables) {
  let cmp = make_component(n_variables);
  let cs = new hd.ConstraintSystem();
  cmp.connectSystem(cs);

  let total = 0;
  let n_samples = 5;
  for (let i = 0; i < n_samples; i++) {
    let start = performance.now();
    cs.update();
    total += performance.now() - start;
  }

  console.log(cmp._name, "&", n_variables, "&", total / n_samples);
}

function bench_component_update_variable(make_component, n_variables) {
  let cmp = make_component(n_variables);
  let cs = new hd.ConstraintSystem();
  cmp.connectSystem(cs);

  let total = 0;
  let n_samples = 5;
  for (let i = 0; i < n_samples; i++) {
    let random_variable;
    let random_number = Math.floor(Math.random() * n_variables);
    for (v of cmp._vars) {
      if (v._index == random_number) {
        random_variable = v;
      }
    }
    let start = performance.now();
    random_variable.set(0);
    total += performance.now() - start;
  }

  console.log(cmp._name, "&", n_variables, "&", total / n_samples);
}

function bench_components(component_makers) {
  for (let n_variables of [1250, 2500, 5000]) {
    for (let make_component of component_makers) {
      // bench_component(make_component, n_variables);
      bench_component_update_variable(make_component, n_variables);
    }
  }
}

bench_components([
  make_empty,
  make_dense,
  make_linear_oneway,
  make_linear_twoway,
  make_ladder,
  make_unprunable
])
