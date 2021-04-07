use hotdrink_rs::{
    algorithms::hierarchical_planner,
    examples::components::{ComponentFactory, Random},
};

fn main() {
    env_logger::init();
    let comp = Random::build::<()>(10000);
    let plan = hierarchical_planner(&comp, &comp.ranking());
    dbg!(plan.is_ok());
}
