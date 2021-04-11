//! An image scaling example.

use hotdrink_rs::{
    component,
    model::{Component, ConstraintSystem},
    ret,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// Generate a type for values in the constraint system.
hotdrink_wasm::component_type_wrapper! {
    pub struct NumberWrapper {
        #[derive(Debug, Clone)]
        pub enum Number {
            i32,
            f64
        }
    }
}

// Generate a wrapper around constraint systems.
hotdrink_wasm::constraint_system_wrapper!(NumberJsCs, NumberWrapper, Number);

fn image_scaling_component() -> Component<Number> {
    component! {
        component ImageScaling {
            let initial_height: i32 = 400, initial_width: i32 = 400,
                relative_height: i32 = 100, relative_width: i32 = 100,
                absolute_height: i32, absolute_width: i32,
                absolute_height_range: i32, absolute_width_range: i32,
                aspect_ratio: f64 = 1.0;

            // relative_height = absolute_height / inital_height
            constraint RelativeHeight {
                a(initial_height: &i32, absolute_height: &i32) -> [relative_height] = ret![100 * absolute_height / initial_height];
                b(initial_height: &i32, relative_height: &i32) -> [absolute_height] = ret![initial_height * relative_height / 100];
            }

            // relative_width = absolute_width / inital_width
            constraint RelativeWidth {
                a(initial_width: &i32, absolute_width: &i32) -> [relative_width] = ret![100 * absolute_width / initial_width];
                b(initial_width: &i32, relative_width: &i32) -> [absolute_width] = ret![initial_width * relative_width / 100];
            }

            constraint AbsoluteHeightRange {
                a(absolute_height: &i32) -> [absolute_height_range] = ret![*absolute_height];
                b(absolute_height_range: &i32) -> [absolute_height] = ret![*absolute_height_range];
            }

            constraint AbsoluteWidthRange {
                a(absolute_width: &i32) -> [absolute_width_range] = ret![*absolute_width];
                b(absolute_width_range: &i32) -> [absolute_width] = ret![*absolute_width_range];
            }

            // aspect_ratio = absolute_width / absolute_height
            constraint AspectRatio {
                c(absolute_height: &i32, absolute_width: &i32) -> [aspect_ratio] = ret![*absolute_width as f64 / *absolute_height as f64];
                a(aspect_ratio: &f64, absolute_height: &i32) -> [absolute_width] = ret![(*aspect_ratio * *absolute_height as f64) as i32];
                b(aspect_ratio: &f64, absolute_width: &i32) -> [absolute_height] = ret![(*absolute_width as f64 / *aspect_ratio) as i32];
            }
        }
    }
}

/// Creates a wrapped ImageScaling example.
#[wasm_bindgen]
pub fn image_scaling() -> Result<NumberJsCs, JsValue> {
    let mut cs = ConstraintSystem::new();
    cs.add_component(image_scaling_component());
    NumberJsCs::wrap(cs)
}