//! An image scaling example.

use hotdrink_rs::{
    component,
    model::{Component, ConstraintSystem},
    ret,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// Generate a type for values in the constraint system.
// The constraint system can only hold values of a single type,
// so we must create an enum [`Number`] that has the variants we need.
// We can not expose non-C-style enums with `wasm-bindgen`, so we must use [`NumberWrapper`]
// to construct the variants instead, using the generated functions `NumberWrapper::i32`
// and `NumberWrapper::f64` from JavaScript.
hotdrink_wasm::component_type_wrapper! {
    pub struct NumberWrapper {
        #[derive(Debug, Clone)]
        pub enum Number {
            i32,
            f64
        }
    }
}

// Generate a wrapper around the constraint system.
// We must specify the generic argument of the constraint system at compile time
// to be able to expose the type with `wasm-bindgen`.
hotdrink_wasm::constraint_system_wrapper!(NumberJsCs, NumberWrapper, Number);

/// Generate a component that describes the constraints
/// required between properties of the image.
pub fn image_resize_component() -> Component<Number> {
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

/// Adds the component to a [`ConstraintSystem`],
/// then wraps that in the [`NumberJsCs`].
#[wasm_bindgen]
pub fn image_resize() -> Result<NumberJsCs, JsValue> {
    let mut cs = ConstraintSystem::new();
    cs.add_component(image_resize_component());
    NumberJsCs::wrap(cs)
}
