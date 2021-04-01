use hotdrink_rs::{
    algorithms::simple_planner::simple_planner, examples::constraint_systems::make_dense_cs,
};

fn main() {
    env_logger::init();
    let cs = make_dense_cs::<()>(1, 100000);
    let comp = cs.component("0").unwrap();
    let plan = simple_planner(comp);
    dbg!(plan.is_some());
}
