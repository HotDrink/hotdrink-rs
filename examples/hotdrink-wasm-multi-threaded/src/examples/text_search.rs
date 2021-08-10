use super::jscs_string::{StringValue, StringJsCs};
use hotdrink_rs::{component, model::ConstraintSystem, ret, util::fib::slow_fib};
use wasm_bindgen::prelude::wasm_bindgen;

pub fn text_search_inner() -> ConstraintSystem<StringValue> {
    let a = component! {
        component Component {
            let input: String, query: String, output: String;
            constraint Filter {
                search(input: &String, query: &String) -> [output] = {
                    slow_fib(40);
                    let output: String = input.lines()
                        .filter(|line| line.contains(query))
                        .collect::<Vec<&str>>()
                        .join("\n");
                    ret![output]
                };
            }
        }
    };

    // Add the component to the constraint system
    let mut cs = ConstraintSystem::new();
    cs.add_component(a);
    cs
}

#[wasm_bindgen]
pub fn text_search() -> StringJsCs {
    StringJsCs::wrap(text_search_inner()).unwrap()
}
