use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
};
use hotdrink_rs::{
    examples::components::{
        ComponentFactory, Ladder, Random, Unprunable, {LinearOneway, LinearTwoway},
    },
    model::Component,
    planner::ComponentSpec,
    planner::{hierarchical_planner, simple_planner},
};
use rand::{distributions::Uniform, prelude::ThreadRng, Rng};

const LINEAR_ONEWAY: fn(usize) -> Component<()> = LinearOneway::build;
const LINEAR_TWOWAY: fn(usize) -> Component<()> = LinearTwoway::build;
const LADDER: fn(usize) -> Component<()> = Ladder::build;
const UNPRUNABLE: fn(usize) -> Component<()> = Unprunable::build;
const RANDOM: fn(usize) -> Component<()> = Random::build;

// Helpers for benching operations on components

fn edit_random(component: &mut Component<()>, rng: &mut ThreadRng, uniform: Uniform<usize>) {
    if component.n_variables() > 0 {
        let random_number: usize = rng.sample(uniform);
        component
            .edit(&format!("var{}", random_number), ())
            .unwrap();
    }
}

fn bench_solve(
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
            edit_random(&mut component, &mut rng, uniform);
            component.solve().unwrap();
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
            edit_random(&mut component, &mut rng, uniform);
            hierarchical_planner(&component).unwrap();
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

fn solve_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve");
    for i in &[0, 500, 5000] {
        bench_solve(&mut group, "linear-oneway", i, LINEAR_ONEWAY);
        bench_solve(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
    }
    for i in &[0, 500, 1000] {
        bench_solve(&mut group, "ladder", i, LADDER);
        bench_solve(&mut group, "random", i, RANDOM);
    }
    for i in &[0, 250, 500] {
        bench_solve(&mut group, "unprunable", i, UNPRUNABLE);
    }
    group.finish();
}

fn hierarchical_planner_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("hierarchical_planner");
    for i in &[0, 500, 5000, 20000] {
        bench_hierarchical_planner(&mut group, "linear-oneway", i, LINEAR_TWOWAY);
        bench_hierarchical_planner(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
    }
    for i in &[0, 500, 1000] {
        bench_hierarchical_planner(&mut group, "ladder", i, LADDER);
        bench_hierarchical_planner(&mut group, "random", i, RANDOM);
    }
    for i in &[0, 250, 500] {
        bench_hierarchical_planner(&mut group, "unprunable", i, UNPRUNABLE);
    }
    group.finish();
}

fn simple_planner_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_planner");
    for i in &[0, 25000, 75000, 100000] {
        bench_simple_planner(&mut group, "linear-oneway", i, LINEAR_TWOWAY);
        bench_simple_planner(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
        bench_simple_planner(&mut group, "ladder", i, LADDER);
        bench_simple_planner(&mut group, "unprunable", i, UNPRUNABLE);
        bench_simple_planner(&mut group, "random", i, RANDOM);
    }
    group.finish();
}

// Benchmarks for testing the limits of the library

fn solve_benches_max(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve_max");
    for i in &[5000] {
        bench_solve(&mut group, "linear-oneway", i, LINEAR_ONEWAY);
        bench_solve(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
    }
    for i in &[1000] {
        bench_solve(&mut group, "ladder", i, LADDER);
        bench_solve(&mut group, "random", i, RANDOM);
    }
    for i in &[500] {
        bench_solve(&mut group, "unprunable", i, UNPRUNABLE);
    }
    group.finish();
}

fn hierarchical_planner_benches_max(c: &mut Criterion) {
    let mut group = c.benchmark_group("hierarchical_planner_max");
    for i in &[20000] {
        bench_hierarchical_planner(&mut group, "linear-oneway", i, LINEAR_TWOWAY);
        bench_hierarchical_planner(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
    }
    for i in &[1000] {
        bench_hierarchical_planner(&mut group, "ladder", i, LADDER);
        bench_hierarchical_planner(&mut group, "random", i, RANDOM);
    }
    for i in &[500] {
        bench_hierarchical_planner(&mut group, "unprunable", i, UNPRUNABLE);
    }
    group.finish();
}

// Benchmarks for generating thesis output

fn thesis_solve(c: &mut Criterion) {
    let mut group = c.benchmark_group("thesis_solve");
    for i in &[100, 500, 1000] {
        bench_solve(&mut group, "linear-oneway", i, LINEAR_ONEWAY);
        bench_solve(&mut group, "linear-twoway", i, LINEAR_TWOWAY);
        bench_solve(&mut group, "ladder", i, LADDER);
        bench_solve(&mut group, "random", i, RANDOM);
        bench_solve(&mut group, "unprunable", i, UNPRUNABLE);
    }
    group.finish();
}

criterion_group!(
    benches,
    solve_benches,
    hierarchical_planner_benches,
    simple_planner_benches,
    solve_benches_max,
    hierarchical_planner_benches_max,
    thesis_solve,
);

criterion_main!(benches);
