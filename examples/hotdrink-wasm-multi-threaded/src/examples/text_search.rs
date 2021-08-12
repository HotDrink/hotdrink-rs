use super::jscs_string::{StringValue, StringJsCs};
use hotdrink_rs::{component, model::ConstraintSystem, ret, util::fib::slow_fib};
use wasm_bindgen::prelude::wasm_bindgen;

pub fn text_search_inner() -> ConstraintSystem<StringValue> {
    let a = component! {
        component Component {
            let input: String = "\
At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga.
Et harum quidem rerum facilis est et expedita distinctio.
Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus.
Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae.
Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.
", query: String, output: String;
            constraint Filter {
                search(input: &String, query: &String) -> [output] = {
                    slow_fib(41);
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
