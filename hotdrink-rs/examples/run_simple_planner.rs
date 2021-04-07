use hotdrink_rs::{
    algorithms::simple_planner,
    examples::components::{ComponentFactory, Random},
};

fn main() {
    env_logger::init();
    let comp = Random::build::<()>(100000);
    let plan = simple_planner(&comp);
    dbg!(plan.is_some());
}
