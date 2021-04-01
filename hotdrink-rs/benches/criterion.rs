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
        random::Random,
        unprunable::Unprunable,
    },
    Component,
};
use rand::{distributions::Uniform, prelude::ThreadRng, Rng};

const LINEAR_ONEWAY: fn(usize) -> Component<()> = LinearOneway::build_component;
const LINEAR_TWOWAY: fn(usize) -> Component<()> = LinearTwoway::build_component;
const LADDER: fn(usize) -> Component<()> = Ladder::build_component;
const UNPRUNABLE: fn(usize) -> Component<()> = Unprunable::build_component;
const RANDOM: fn(usize) -> Component<()> = Random::build_component;

// Helpers for benching operations on components

fn update_random(component: &mut Component<()>, rng: &mut ThreadRng, uniform: Uniform<usize>) {
    if component.n_variables() > 0 {
        let random_number: usize = rng.sample(uniform);
        component
            .set_variable(&format!("var{}", random_number), ())
            .unwrap();
    }
}

fn bench_update(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    input: &usize,
    make_component: fn(usize) -> Component<()>,
) {
    group.bench_with_input(BenchmarkId::new(name, input), input, |b, input| {
        let mut component: Component<()> = make_component(*input);
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new_inclusive(0, component.n_variables().saturating_sub(1));
        b.iter(|| {
            update_random(&mut component, &mut rng, uniform);
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
        let mut component = make_component(*input);
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new_inclusive(0, component.n_variables().saturating_sub(1));
        b.iter(|| {
            update_random(&mut component, &mut rng, uniform);
            hierarchical_planner(&component, &component.ranking()).unwrap();
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
    for i in &[250, 500, 1000] {
        bench_hierarchical_planner(&mut group, "unprunable", i, UNPRUNABLE);
        bench_hierarchical_planner(&mut group, "random", i, RANDOM);
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
