use hotdrink_rs::examples::components::{ComponentFactory, LinearOneway};

fn main() {
    env_logger::init();
    let mut component = LinearOneway::build::<()>(10000);
    let result = component.solve();
    println!("{:?}", result);
}
