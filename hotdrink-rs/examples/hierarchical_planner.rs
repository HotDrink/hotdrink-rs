use hotdrink_rs::{
    algorithms::hierarchical_planner::hierarchical_planner, data::traits::ComponentSpec,
    examples::constraint_systems::make_dense_cs,
};

fn main() {
    env_logger::init();
    let cs = make_dense_cs::<()>(1, 100000);
    let comp = cs.component("0").unwrap();
    let ranking: Vec<_> = (0..comp.n_variables()).collect();
    let plan = hierarchical_planner(comp, &ranking);
    dbg!(plan.is_ok());
}
