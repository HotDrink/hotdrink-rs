use hotdrink_rs::{examples::components::numbers::sum, model::Component, Event};

fn main() {
    let mut comp: Component<i32> = sum();

    println!("Subscribing");
    comp.subscribe("a", |e| {
        if let Event::Ready(v) = e {
            println!("a = {}", v);
        }
    })
    .unwrap();

    comp.subscribe("b", |e| {
        if let Event::Ready(v) = e {
            println!("b = {}", v);
        }
    })
    .unwrap();

    comp.subscribe("c", |e| {
        if let Event::Ready(v) = e {
            println!("c = {}", v);
        }
    })
    .unwrap();

    println!("Setting a");
    comp.set_variable("a", 3).unwrap();

    println!("Updating");
    comp.update().unwrap();

    println!("Setting b");
    comp.set_variable("b", 5).unwrap();

    println!("Updating");
    comp.update().unwrap();
}
