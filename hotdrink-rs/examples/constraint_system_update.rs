use hotdrink_rs::{
    examples::constraint_systems::linear::linear_twoway, thread::dummy_pool::DummyPool,
};

fn main() {
    env_logger::init();
    let mut cs = linear_twoway::<()>(1, 100000);
    let result = cs.par_update(&mut DummyPool);
    println!("{:?}", result);
}
