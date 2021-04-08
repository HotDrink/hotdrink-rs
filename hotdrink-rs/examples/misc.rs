use hotdrink_rs::{examples::components::numbers::sum, model::Component};

fn main() {
    let comp: Component<i32> = sum();
    dbg!(comp);
}
