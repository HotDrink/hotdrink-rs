//! A constraint system example that demonstrates a case where we should adjust the priorities of variables.

use super::jscs_string::StringJsCs;
use hotdrink_rs::{component, model::ConstraintSystem, ret};
use wasm_bindgen::prelude::wasm_bindgen;

/// Constructs a [`StringJsCs`].
#[wasm_bindgen]
pub fn priority_adjust() -> StringJsCs {
    let mut cs = ConstraintSystem::new();
    cs.add_component(component! {
        component PriorityAdjust {
            let a: String = "a", b: String = "b", c: String = "c", d: String = "d";
            constraint Ab {
                m1(a: &String) -> [b] = ret![a.to_string()];
                m2(b: &String) -> [a] = ret![b.to_string()];
            }
            constraint Bcd {
                m3(b: &String, c: &String) -> [d] = ret![b.to_string() + " & " + &c.to_string()];
                m4(d: &String) -> [b, c] = ret![d.to_string(), d.to_string()];
            }
        }
    });
    StringJsCs::wrap(cs).unwrap()
}
