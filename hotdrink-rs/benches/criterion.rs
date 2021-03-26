use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
};
use hotdrink_rs::{
    algorithms::{hierarchical_planner::hierarchical_planner, simple_planner::simple_planner},
    data::{constraint_system::ConstraintSystem, traits::ComponentSpec},
    examples::{
        components::random::make_random,
        constraint_systems::{
            dense::make_dense_cs,
            empty::make_empty_cs,
            ladder::ladder,
            linear::linear_twoway,
            linear_oneway,
            sparse::make_sparse_cs,
            tree::{multioutput_threeway, unprunable},
        },
    },
    thread::dummy_pool::DummyPool,
    Component,
};
use rand::Rng;

// Update constraint systems
fn bench_cs_update(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_cs: fn(usize, usize) -> ConstraintSystem<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let mut cs: ConstraintSystem<()> = make_cs(1, *input);
        let mut dummy_pool = DummyPool;
        b.iter(|| {
            cs.par_update_always(&mut dummy_pool).unwrap();
        })
    });
}

// Update constraint systems
fn bench_cs_update_with_modified_variable(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_cs: fn(usize, usize) -> ConstraintSystem<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let mut cs: ConstraintSystem<()> = make_cs(1, *input);
        let mut dummy_pool = DummyPool;
        let mut rng = rand::thread_rng();
        let uniform = rand::distributions::Uniform::new_inclusive(0, input.saturating_sub(1));
        b.iter(|| {
            if *input > 0 {
                let random_number: usize = rng.sample(uniform);
                cs.set_variable("0", &format!("var{}", random_number), ());
            }
            cs.par_update(&mut dummy_pool).unwrap();
        })
    });
}

fn constraint_system_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("constraint_system_update");
    for i in &[1250, 2500, 5000] {
        bench_cs_update(&mut group, "dense", i, make_dense_cs);
        bench_cs_update(&mut group, "empty", i, make_empty_cs);
        bench_cs_update(&mut group, "linear/oneway", i, linear_oneway);
        bench_cs_update(&mut group, "linear/twoway", i, linear_twoway);
        // bench_cs_update(&mut group, "sparse", i, make_sparse_cs);
        // bench_cs_update(
        //     &mut group,
        //     "tree/multioutput/threeway",
        //     i,
        //     multioutput_threeway,
        // );
        bench_cs_update(&mut group, "ladder", i, ladder);
    }
    for i in &[1250, 2500, 5000] {
        bench_cs_update(&mut group, "unprunable", i, unprunable);
    }
    group.finish();
}

fn constraint_system_update_with_modified_variable(c: &mut Criterion) {
    let mut group = c.benchmark_group("constraint_system_update_with_modified_variable");
    for i in &[1250, 2500, 5000] {
        bench_cs_update_with_modified_variable(&mut group, "dense", i, make_dense_cs);
        bench_cs_update_with_modified_variable(&mut group, "empty", i, make_empty_cs);
        bench_cs_update_with_modified_variable(&mut group, "linear/oneway", i, linear_oneway);
        bench_cs_update_with_modified_variable(&mut group, "linear/twoway", i, linear_twoway);
        //     bench_cs_update_with_modified_variable(&mut group, "sparse", i, make_sparse_cs);
        //     bench_cs_update_with_modified_variable(
        //         &mut group,
        //         "tree/multioutput/threeway",
        //         i,
        //         multioutput_threeway,
        //     );
    }
    for i in &[1250, 2500, 5000] {
        bench_cs_update_with_modified_variable(&mut group, "unprunable", i, unprunable);
        bench_cs_update_with_modified_variable(&mut group, "ladder", i, ladder);
    }
    group.finish();
}

// Plan components

fn bench_hierarchical_planner(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_cs: fn(usize, usize) -> ConstraintSystem<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let cs: ConstraintSystem<()> = make_cs(1, *input);
        let comp = cs.get_component("0");
        let ranking: Vec<usize> = (0..comp.n_variables()).collect();
        b.iter(|| {
            hierarchical_planner(comp, &ranking).unwrap();
        })
    });
}

fn bench_hierarchical_planner_component(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_component: fn(usize) -> Component<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let comp = make_component(*input);
        let ranking: Vec<usize> = (0..comp.n_variables()).collect();
        b.iter(|| {
            hierarchical_planner(&comp, &ranking).unwrap();
        })
    });
}

fn component_hierarchical_planner(c: &mut Criterion) {
    let mut group = c.benchmark_group("component_hierarchical_planner");
    for i in &[0, 1000, 5000, 10000, 20000] {
        bench_hierarchical_planner(&mut group, "dense", i, make_dense_cs);
        bench_hierarchical_planner(&mut group, "empty", i, make_empty_cs);
        bench_hierarchical_planner(&mut group, "linear/twoway", i, linear_twoway);
        bench_hierarchical_planner(&mut group, "sparse", i, make_sparse_cs);
        bench_hierarchical_planner(
            &mut group,
            "tree/multioutput/threeway",
            i,
            multioutput_threeway,
        );
        bench_hierarchical_planner(&mut group, "ladder", i, ladder);
    }
    for i in &[0, 250, 500, 1000] {
        bench_hierarchical_planner(&mut group, "unprunable", i, unprunable);
        bench_hierarchical_planner_component(&mut group, "random", i, |nv| {
            make_random((nv as f64 * 0.75) as usize, 3)
        });
    }
    group.finish();
}

fn bench_simple_planner(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_cs: fn(usize, usize) -> ConstraintSystem<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let cs = make_cs(1, *input);
        let comp = cs.get_component("0");
        b.iter(|| {
            simple_planner(comp).unwrap();
        })
    });
}

fn bench_simple_planner_component(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_component: fn(usize) -> Component<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let comp = make_component(*input);
        b.iter(|| {
            simple_planner(&comp).unwrap();
        })
    });
}

fn component_simple_planner(c: &mut Criterion) {
    let mut group = c.benchmark_group("component_simple_planner");
    for i in &[0, 1000, 5000, 10000, 20000] {
        bench_simple_planner(&mut group, "dense", i, make_dense_cs);
        bench_simple_planner(&mut group, "empty", i, make_empty_cs);
        bench_simple_planner(&mut group, "linear/twoway", i, linear_twoway);
        bench_simple_planner(&mut group, "ladder", i, ladder);
        bench_simple_planner(&mut group, "unprunable", i, unprunable);
        bench_simple_planner_component(&mut group, "random", i, |nv| {
            make_random((nv as f64 * 0.75) as usize, 3)
        });
    }
    group.finish();
}

fn max_simple_planner(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_simple_planner");
    for i in &[150000] {
        bench_simple_planner(&mut group, "empty", i, make_empty_cs);
        bench_simple_planner(&mut group, "dense", i, make_dense_cs);
        bench_simple_planner(&mut group, "linear/oneway", i, linear_oneway);
        bench_simple_planner(&mut group, "linear/twoway", i, linear_twoway);
        bench_simple_planner(&mut group, "ladder", i, ladder);
        bench_simple_planner(&mut group, "unprunable", i, unprunable);
    }
    group.finish();
}

fn max_hierarchical_planner(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_hierarchical_planner");
    for i in &[30000] {
        bench_hierarchical_planner(&mut group, "empty", i, make_empty_cs);
        bench_hierarchical_planner(&mut group, "dense", i, make_dense_cs);
        bench_hierarchical_planner(&mut group, "linear/oneway", i, linear_oneway);
        bench_hierarchical_planner(&mut group, "linear/twoway", i, linear_twoway);
        bench_hierarchical_planner(&mut group, "ladder", i, ladder);
    }
    for i in &[500] {
        bench_hierarchical_planner(&mut group, "unprunable", i, unprunable);
    }
    group.finish();
}

fn max_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_update");
    for i in &[20000] {
        bench_cs_update(&mut group, "empty", i, make_empty_cs);
        bench_cs_update(&mut group, "dense", i, make_dense_cs);
        bench_cs_update(&mut group, "linear/oneway", i, linear_oneway);
        bench_cs_update(&mut group, "linear/twoway", i, linear_twoway);
        bench_cs_update(&mut group, "ladder", i, ladder);
    }
    for i in &[500] {
        bench_cs_update(&mut group, "unprunable", i, unprunable);
    }
    group.finish();
}

fn max_update_with_modified_variable(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_update_with_modified_variable");
    for i in &[20000] {
        bench_cs_update_with_modified_variable(&mut group, "empty", i, make_empty_cs);
        bench_cs_update_with_modified_variable(&mut group, "dense", i, make_dense_cs);
        bench_cs_update_with_modified_variable(&mut group, "linear/oneway", i, linear_oneway);
        bench_cs_update_with_modified_variable(&mut group, "linear/twoway", i, linear_twoway);
    }
    for i in &[750] {
        bench_cs_update_with_modified_variable(&mut group, "ladder", i, ladder);
        bench_cs_update_with_modified_variable(&mut group, "unprunable", i, unprunable);
    }
    group.finish();
}

criterion_group!(
    benches,
    constraint_system_update,
    constraint_system_update_with_modified_variable,
    component_hierarchical_planner,
    component_simple_planner,
    max_simple_planner,
    max_hierarchical_planner,
    max_update,
    max_update_with_modified_variable
);

criterion_main!(benches);
