use hotdrink_rs::examples::components::{ComponentFactory, Random};

fn main() {
    env_logger::init();
    let mut component = Random::build::<()>(10000);
    let result = component.solve();
    println!("{:?}", result);
}
