use hotdrink_rs::{event::Event, examples::components::numbers::sum, model::Component};

fn main() {
    let mut comp: Component<i32> = sum();

    println!("Subscribing");
    comp.subscribe("a", |e| {
        if let Event::Ready(v) = e {
            println!("a = {:?}", v);
        }
    })
    .unwrap();

    comp.subscribe("b", |e| {
        if let Event::Ready(v) = e {
            println!("b = {:?}", v);
        }
    })
    .unwrap();

    comp.subscribe("c", |e| {
        if let Event::Ready(v) = e {
            println!("c = {:?}", v);
        }
    })
    .unwrap();

    println!("Setting a");
    comp.edit("a", 3).unwrap();

    println!("Updating");
    comp.solve().unwrap();

    println!("Setting b");
    comp.edit("b", 5).unwrap();

    println!("Updating");
    comp.solve().unwrap();
}
