use hotdrink_rs::{
    data::constraint_system::ConstraintSystem,
    examples::constraint_systems::{
        empty::make_empty_cs,
        ladder::ladder,
        linear::{linear_oneway, linear_twoway},
        make_dense_cs,
        tree::unprunable,
    },
    thread::dummy_pool::DummyPool,
};
use std::fmt::Debug;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn constraint_system_update<T>(
    name: &str,
    make_cs: impl Fn(usize, usize) -> ConstraintSystem<T>,
    n_variables: usize,
) where
    T: Debug + Clone + Default + Send + 'static,
{
    let performance = web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance");

    let n_samples = 5;
    let mut total_time: f64 = 0.0;
    for _ in 0..n_samples {
        let mut cs = make_cs(1, n_variables);
        // web_sys::console::time_with_label(&format!("{} & {}", name, n_variables));
        let start = performance.now();
        if n_variables > 0 {
            let random_number = (js_sys::Math::random() * n_variables as f64) as usize;
            cs.set_variable("0", &format!("var{}", random_number), T::default());
        }
        let result = cs.par_update(&mut DummyPool);
        total_time += performance.now() - start;
        // web_sys::console::time_end_with_label(&format!("{} & {}", name, n_variables));
        assert_eq!(result, Ok(()));
    }

    console_log!(
        "{} & {} & {}",
        name,
        n_variables,
        total_time / n_samples as f64
    );
}

fn constraint_system_update_with_modified_variable<T>(
    name: &str,
    make_cs: impl Fn(usize, usize) -> ConstraintSystem<T>,
    n_variables: usize,
) where
    T: Debug + Clone + Default + Send + 'static,
{
    let performance = web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance");

    let n_samples = 5;
    let mut total_time: f64 = 0.0;
    for _ in 0..n_samples {
        let mut cs = make_cs(1, n_variables);
        // web_sys::console::time_with_label(&format!("{} & {}", name, n_variables));
        let start = performance.now();
        if n_variables > 0 {
            let random_number = (js_sys::Math::random() * n_variables as f64) as usize;
            cs.set_variable("0", &format!("var{}", random_number), T::default());
        }
        let result = cs.par_update(&mut DummyPool);
        total_time += performance.now() - start;
        // web_sys::console::time_end_with_label(&format!("{} & {}", name, n_variables));
        assert_eq!(result, Ok(()));
    }

    console_log!(
        "{} & {} & {}",
        name,
        n_variables,
        total_time / n_samples as f64
    );
}

#[wasm_bindgen_test]
fn bench_constraint_system_update() {
    for &i in &[1250, 2500, 5000] {
        constraint_system_update("empty", make_empty_cs::<i32>, i);
        constraint_system_update("dense", make_dense_cs::<i32>, i);
        constraint_system_update("linear-oneway", linear_oneway::<i32>, i);
        constraint_system_update("linear-twoway", linear_twoway::<i32>, i);
        constraint_system_update("ladder", ladder::<i32>, i);
        constraint_system_update("unprunable", unprunable::<i32>, i);
    }
}

#[wasm_bindgen_test]
fn bench_constraint_system_update_with_modified_variable() {
    for &i in &[1250, 2500, 5000] {
        constraint_system_update_with_modified_variable("empty", make_empty_cs::<i32>, i);
        constraint_system_update_with_modified_variable("dense", make_dense_cs::<i32>, i);
        constraint_system_update_with_modified_variable("linear-oneway", linear_oneway::<i32>, i);
        constraint_system_update_with_modified_variable("linear-twoway", linear_twoway::<i32>, i);
        constraint_system_update_with_modified_variable("ladder", ladder::<i32>, i);
        constraint_system_update_with_modified_variable("unprunable", unprunable::<i32>, i);
    }
}
