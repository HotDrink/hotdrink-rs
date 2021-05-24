//! An example of when scheduling does not happen optimally.
//! Solving the following component in parallel will schedule
//! methods in a way that blocks some threads until their
//! dependencies are computed.

use hotdrink_rs::{component, planner::hierarchical_planner, ret};

fn main() {
    let component = component! {
        component Component {
            let a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32, j: i32;

            // Start
            constraint Abc {
                m1(a: &i32) -> [b, c] = ret![*a, *a];
            }

            // Red path
            constraint Bd {
                bd(b: &i32) -> [d] = ret![*b];
            }

            constraint Dg {
                bc(d: &i32) -> [g] = ret![*d];
            }

            // Green path
            constraint Be {
                de(b: &i32) -> [e] = ret![*b];
            }

            constraint Eh {
                de(e: &i32) -> [h] = ret![*e];
            }

            // Blue path
            constraint Cf {
                gh(c: &i32) -> [f] = ret![*c];
            }

            constraint Fi {
                de(f: &i32) -> [i] = ret![*f];
            }

            // Last part
            constraint Ghij {
                ghij(g: &i32, h: &i32, i: &i32) -> [j] = ret![g + h + i];
            }
        }
    };

    let plan = hierarchical_planner(&component).unwrap();
    dbg!(plan);
}
