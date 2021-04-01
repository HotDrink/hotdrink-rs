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
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn bench_update<Cb: ComponentFactory>(n_constraints: usize) {
    let performance = web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance");

    let n_samples = 5;
    let mut total_time: f64 = 0.0;
    for _ in 0..n_samples {
        let mut component: Component<()> = Cb::build(n_constraints);
        // web_sys::console::time_with_label(&format!("{} & {}", name, n_variables));
        let start = performance.now();
        if n_constraints > 0 {
            let random_number = (js_sys::Math::random() * n_constraints as f64) as usize;
            component
                .set_variable(&format!("var{}", random_number), ())
                .unwrap();
        }
        let result = component.par_update(&mut DummyPool);
        total_time += performance.now() - start;
        // web_sys::console::time_end_with_label(&format!("{} & {}", name, n_variables));
        assert_eq!(result, Ok(()));
    }

    console_log!(
        "{} & {} & {}",
        Cb::name(),
        n_constraints,
        total_time / n_samples as f64
    );
}

#[wasm_bindgen_test]
fn thesis_update() {
    for &i in &[1250, 2500, 5000] {
        bench_update::<LinearOneway>(i);
        bench_update::<LinearTwoway>(i);
        bench_update::<Ladder>(i);
        bench_update::<Unprunable>(i);
        bench_update::<Random>(i);
    }
}
