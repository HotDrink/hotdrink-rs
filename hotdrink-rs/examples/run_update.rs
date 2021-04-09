use hotdrink_rs::examples::components::{ComponentFactory, Random};

fn main() {
    env_logger::init();
    let mut component = Random::build::<()>(100000);
    let result = component.update();
    println!("{:?}", result);
}