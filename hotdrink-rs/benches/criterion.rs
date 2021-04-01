use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
};
use hotdrink_rs::{
    algorithms::{hierarchical_planner::hierarchical_planner, simple_planner::simple_planner},
    data::traits::ComponentSpec,
    examples::components::{
        factory::ComponentFactory,
        ladder::Ladder,
        linear::{LinearOneway, LinearTwoway},
        random::RandomComponentFactory,
        unprunable::Unprunable,
    },
    Component,
};
use rand::Rng;

const LINEAR_ONEWAY: fn(usize) -> Component<()> = LinearOneway::build_component;
const LINEAR_TWOWAY: fn(usize) -> Component<()> = LinearTwoway::build_component;
const LADDER: fn(usize) -> Component<()> = Ladder::build_component;
const UNPRUNABLE: fn(usize) -> Component<()> = Unprunable::build_component;
const RANDOM: fn(usize) -> Component<()> = RandomComponentFactory::build_component;

// Helpers for benching operations on components

fn bench_update(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_component: fn(usize) -> Component<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let mut component: Component<()> = make_component(*input);
        let mut rng = rand::thread_rng();
        let uniform = rand::distributions::Uniform::new(0, component.n_variables());
        b.iter(|| {
            if *input > 0 {
                let random_number: usize = rng.sample(uniform);
                component
                    .set_variable(&format!("var{}", random_number), ())
                    .unwrap();
            }
            component.update().unwrap();
        })
    });
}

fn bench_hierarchical_planner(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_component: fn(usize) -> Component<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let mut comp = make_component(*input);
        let mut rng = rand::thread_rng();
        let uniform = rand::distributions::Uniform::new(0, comp.n_variables());
        b.iter(|| {
            if comp.n_variables() > 0 {
                let random_number: usize = rng.sample(uniform);
                comp.set_variable(&format!("var{}", random_number), ())
                    .unwrap();
            }
            hierarchical_planner(&comp, &comp.ranking()).unwrap();
        })
    });
}

fn bench_simple_planner(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_component: fn(usize) -> Component<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let component = make_component(*input);
        b.iter(|| {
            simple_planner(&component).unwrap();
        })
    });
}

// General benchmarks for components

fn update_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("update");
    for i in &[1250, 2500, 5000] {
        bench_update(&mut group, "linear-oneway", i, LINEAR_ONEWAY);
        bench_update(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
        bench_update(&mut group, "ladder", i, LADDER);
        bench_update(&mut group, "unprunable", i, UNPRUNABLE);
        bench_update(&mut group, "random", i, RANDOM);
    }
    group.finish();
}

fn hierarchical_planner_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("hierarchical_planner");
    for i in &[0, 1000, 5000, 10000, 20000] {
        bench_hierarchical_planner(&mut group, "linear-oneway", i, LINEAR_TWOWAY);
        bench_hierarchical_planner(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
        bench_hierarchical_planner(&mut group, "ladder", i, LADDER);
    }
    for i in &[0, 250, 500, 1000] {
        bench_hierarchical_planner(&mut group, "unprunable", i, UNPRUNABLE);
        bench_hierarchical_planner(&mut group, "random", i, RANDOM);
    }
    group.finish();
}

fn simple_planner_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_planner");
    for i in &[0, 1000, 5000, 10000, 20000] {
        bench_simple_planner(&mut group, "linear-oneway", i, LINEAR_TWOWAY);
        bench_simple_planner(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
        bench_simple_planner(&mut group, "ladder", i, LADDER);
        bench_simple_planner(&mut group, "unprunable", i, UNPRUNABLE);
        bench_simple_planner(&mut group, "random", i, RANDOM);
    }
    group.finish();
}

// Benchmarks for testing the limits of the library

fn update_benches_max(c: &mut Criterion) {
    let mut group = c.benchmark_group("update_max");
    for i in &[20000] {
        bench_update(&mut group, "linear-oneway", i, LINEAR_ONEWAY);
        bench_update(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
    }
    for i in &[750] {
        bench_update(&mut group, "ladder", i, LADDER);
        bench_update(&mut group, "unprunable", i, UNPRUNABLE);
        bench_update(&mut group, "random", i, RANDOM);
    }
    group.finish();
}

fn hierarchical_planner_benches_max(c: &mut Criterion) {
    let mut group = c.benchmark_group("hierarchical_planner_max");
    for i in &[30000] {
        bench_hierarchical_planner(&mut group, "linear-oneway", i, LINEAR_ONEWAY);
        bench_hierarchical_planner(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
        bench_hierarchical_planner(&mut group, "ladder", i, LADDER);
    }
    for i in &[500] {
        bench_hierarchical_planner(&mut group, "unprunable", i, UNPRUNABLE);
        bench_hierarchical_planner(&mut group, "random", i, RANDOM);
    }
    group.finish();
}

fn simple_planner_benches_max(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_planner_max");
    for i in &[150000] {
        bench_simple_planner(&mut group, "linear-oneway", i, LINEAR_ONEWAY);
        bench_simple_planner(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
        bench_simple_planner(&mut group, "ladder", i, LADDER);
        bench_simple_planner(&mut group, "unprunable", i, UNPRUNABLE);
        bench_simple_planner(&mut group, "random", i, RANDOM);
    }
    group.finish();
}

// Benchmarks for generating thesis output

fn thesis_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("thesis_update");
    for i in &[250, 500, 1000] {
        bench_update(&mut group, "linear-oneway", i, LINEAR_ONEWAY);
        bench_update(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
        bench_update(&mut group, "ladder", i, LADDER);
        bench_update(&mut group, "unprunable", i, UNPRUNABLE);
        bench_update(&mut group, "random", i, RANDOM);
    }
    group.finish();
}

criterion_group!(
    benches,
    update_benches,
    hierarchical_planner_benches,
    simple_planner_benches,
    update_benches_max,
    hierarchical_planner_benches_max,
    simple_planner_benches_max,
    thesis_update,
);

criterion_main!(benches);
