use hotdrink_rs::{
    examples::components::{
        ComponentFactory, Ladder, LinearOneway, LinearTwoway, Random, Unprunable,
    },
    model::Component,
    planner::ComponentSpec,
};
use rand::{distributions::Uniform, prelude::ThreadRng, Rng};

fn update_random(component: &mut Component<()>, rng: &mut ThreadRng, uniform: Uniform<usize>) {
    if component.n_variables() > 0 {
        let random_number: usize = rng.sample(uniform);
        component
            .set_variable(&format!("var{}", random_number), ())
            .unwrap();
    }
}

fn bench_update_max<Cb: ComponentFactory>(n_constraints: usize) {
    let n_samples = 5;
    let mut max = std::time::Duration::from_secs(0);
    for _ in 0..n_samples {
        let mut component: Component<()> = Cb::build(n_constraints);
        let mut rng = rand::thread_rng();
        let uniform = Uniform::new_inclusive(0, component.n_variables().saturating_sub(1));
        // web_sys::console::time_with_label(&format!("{} & {}", name, n_variables));
        let start = std::time::Instant::now();
        if n_constraints > 0 {
            update_random(&mut component, &mut rng, uniform);
        }
        let result = component.update();
        let end = std::time::Instant::now();
        max = max.max(end - start);
        // web_sys::console::time_end_with_label(&format!("{} & {}", name, n_variables));
        assert_eq!(result, Ok(()));
    }

    println!("{} & {} & {}", Cb::name(), n_constraints, max.as_millis());
}

fn main() {
    for &i in &[100, 500, 1000] {
        bench_update_max::<LinearOneway>(i);
        bench_update_max::<LinearTwoway>(i);
        bench_update_max::<Ladder>(i);
        bench_update_max::<Random>(i);
        bench_update_max::<Unprunable>(i);
    }
}
