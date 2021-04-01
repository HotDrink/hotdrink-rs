use hotdrink_rs::{
    examples::components::{
        ladder::Ladder,
        linear::{LinearOneway, LinearTwoway},
        random::Random,
        unprunable::Unprunable,
        ComponentFactory,
    },
    thread::dummy_pool::DummyPool,
    Component,
};
use std::fmt::Debug;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const LINEAR_ONEWAY: fn(usize) -> Component<()> = LinearOneway::build_component;
const LINEAR_TWOWAY: fn(usize) -> Component<()> = LinearTwoway::build_component;
const LADDER: fn(usize) -> Component<()> = Ladder::build_component;
const UNPRUNABLE: fn(usize) -> Component<()> = Unprunable::build_component;
const RANDOM: fn(usize) -> Component<()> = Random::build_component;

fn bench_update<T, F>(name: &str, make_cs: F, n_constraints: usize)
where
    T: Debug + Clone + Default + Send + 'static,
    F: Fn(usize) -> Component<T>,
{
    let performance = web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance");

    let n_samples = 5;
    let mut total_time: f64 = 0.0;
    for _ in 0..n_samples {
        let mut cs = make_cs(n_constraints);
        // web_sys::console::time_with_label(&format!("{} & {}", name, n_variables));
        let start = performance.now();
        if n_constraints > 0 {
            let random_number = (js_sys::Math::random() * n_constraints as f64) as usize;
            cs.set_variable(&format!("var{}", random_number), T::default())
                .unwrap();
        }
        let result = cs.par_update(&mut DummyPool);
        total_time += performance.now() - start;
        // web_sys::console::time_end_with_label(&format!("{} & {}", name, n_variables));
        assert_eq!(result, Ok(()));
    }

    console_log!(
        "{} & {} & {}",
        name,
        n_constraints,
        total_time / n_samples as f64
    );
}

#[wasm_bindgen_test]
fn thesis_update() {
    for &i in &[1250, 2500, 5000] {
        bench_update("linear-oneway", LINEAR_ONEWAY, i);
        bench_update("linear-twoway", LINEAR_TWOWAY, i);
        bench_update("ladder", LADDER, i);
        bench_update("unprunable", UNPRUNABLE, i);
        bench_update("random", RANDOM, i);
    }
}
