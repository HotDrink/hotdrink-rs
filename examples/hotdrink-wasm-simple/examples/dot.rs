use hotdrink_wasm_simple::image_resize::image_resize_component;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("image_resize_component.dot").unwrap();
    write!(
        file,
        "{}",
        image_resize_component().to_dot_detailed().unwrap()
    )
    .unwrap();
}
