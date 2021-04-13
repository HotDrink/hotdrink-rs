use super::jscs_number::I32JsCs;
use hotdrink_rs::{component, model::ConstraintSystem, ret};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum() -> I32JsCs {
    let mut cs = ConstraintSystem::new();
    cs.add_component(component! {
        component Sum {
            let a: i32 = 0, b: i32 = 0, c: i32 = 0;
            constraint Sum {
                abc(a: &i32, b: &i32) -> [c] = ret![*a + *b];
                acb(a: &i32, c: &i32) -> [b] = ret![*c - *a];
                bca(b: &i32, c: &i32) -> [a] = ret![*c - *b];
            }
        }
    });
    I32JsCs::wrap(cs).unwrap()
}
