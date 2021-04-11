function _make_linear_oneway(builder, n_constraints) {
    for (let i = 0; i < n_constraints + 1; i++) {
        builder.add_variable(i);
    }
    for (let i = 0; i < n_constraints; i++) {
        builder.add_constraint([i, i + 1]);
        builder.add_method([i], [i + 1], "m1");
    }
}

function _make_linear_twoway(builder, n_constraints) {
    for (let i = 0; i < n_constraints + 1; i++) {
        builder.add_variable(i);
    }
    for (let i = 0; i < n_constraints; i++) {
        builder.add_constraint([i, i + 1]);
        builder.add_method([i], [i + 1], "m1");
        builder.add_method([i + 1], [i], "m2");
    }
}