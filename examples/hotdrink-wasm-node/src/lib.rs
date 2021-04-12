use hotdrink_rs::{component, model::ConstraintSystem, ret};
use wasm_bindgen::prelude::*;

hotdrink_wasm::component_type_wrapper! {
    pub struct NumberWrapper {
        #[derive(Clone, Debug)]
        pub enum Number {
            i32
        }
    }
}

hotdrink_wasm::constraint_system_wrapper! {
    NumberConstraintSystem,
    NumberWrapper,
    Number
}

#[wasm_bindgen]
pub fn sum() -> NumberConstraintSystem {
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
    NumberConstraintSystem::wrap(cs).unwrap()
}
