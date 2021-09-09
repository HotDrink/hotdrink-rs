use hotdrink_rs::model::Component;
use hotdrink_wasm_simple::image_scaling::image_scaling_component;
use hotdrink_wasm_simple::rectangles::rectangles_component;
use std::io::Write;

fn write_comp<T>(name: &str, component: Component<T>) {
    let mut file = std::fs::File::create(name).unwrap();
    write!(
        file,
        "{}",
        component.to_dot_detailed().unwrap()
    )
    .unwrap();
}

fn write_comp_simple<T>(name: &str, component: Component<T>) {
    let mut file = std::fs::File::create(name).unwrap();
    write!(
        file,
        "{}",
        component.to_dot_simple().unwrap()
    )
    .unwrap();
}

fn main() {
    write_comp("image_resize_component.dot", image_scaling_component());
    write_comp_simple("image_resize_component_simple.dot", image_scaling_component());
    write_comp("rectangles_component.dot", rectangles_component());
}
