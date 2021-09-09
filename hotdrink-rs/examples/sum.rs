use hotdrink_rs::{event::Event, examples::components::numbers::sum, model::Component};

fn main() {
    let mut comp: Component<i32> = sum();

    eprintln!("Subscribing");
    comp.subscribe("a", |e| {
        if let Event::Ready(v) = e {
            eprintln!("a = {:?}", v);
        }
    })
    .unwrap();

    comp.subscribe("b", |e| {
        if let Event::Ready(v) = e {
            eprintln!("b = {:?}", v);
        }
    })
    .unwrap();

    comp.subscribe("c", |e| {
        if let Event::Ready(v) = e {
            eprintln!("c = {:?}", v);
        }
    })
    .unwrap();

    eprintln!("Setting a");
    comp.edit("a", 3).unwrap();

    eprintln!("Updating");
    comp.solve().unwrap();

    eprintln!("Setting b");
    comp.edit("b", 5).unwrap();

    eprintln!("Updating");
    comp.solve().unwrap();
}
