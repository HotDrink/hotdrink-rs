//! A constraint system example that shows off multiple features.

use super::circle::Circle;
use crate::thread::pool::StaticPool;
use hotdrink_rs::{
    algorithms::fib::slow_fib, component, data::constraint_system::ConstraintSystem, fail, ret,
    thread::thread_pool::TerminationStrategy,
};
use wasm_bindgen::prelude::wasm_bindgen;

type DbResult = Option<i32>;

crate::gen_js_val! {
    pub MyValueWrapper {
        #[derive(Debug, Clone)]
        pub MyValue {
            f64,
            String,
            Circle,
            DbResult
        }
    }
}

crate::gen_js_constraint_system!(
    JsConstraintSystem,
    MyValueWrapper,
    MyValue,
    StaticPool,
    2,
    TerminationStrategy::UnusedResultAndNotDone
);

/// Constructs a new [`ConstraintSystem<MyValue>`].
pub fn demo_cs_inner() -> ConstraintSystem<MyValue> {
    let arithmetic = component! {
        component Arithmetic {
            let a: f64 = 2, b: f64 = 4, c: f64 = 0, d: f64 = 0;

            constraint Sum {
                s1(a: &f64, b: &f64) -> [c] = ret![a + b];
                s2(b: &f64, c: &f64) -> [a] = ret![b - c];
                s3(c: &f64, a: &f64) -> [b] = ret![c - a];
            }

            constraint Product {
                p1(a: &f64, b: &f64) -> [d] = ret![a * b];
                p2(b: &f64, d: &f64) -> [a] = ret![d / b];
                p3(d: &f64, a: &f64) -> [b] = ret![d / a];
            }
        }
    };

    let concat = component! {
        component concat {
            let e: String = "Hello", f: String = "world", g: String = "";
            constraint concat {
                concat(e: &String, f: &String) -> [g] = ret![e.clone() + " " + f];
            }
        }
    };

    let fib = component! {
        component fib {
            let fib_in: f64 = 40, fib_out: f64 = 0, fib_in_slider: f64 = 40;
            constraint fib {
                fib(fib_in: &f64) -> [fib_out] = ret![slow_fib(*fib_in as i32) as f64];
            }
            constraint Slider {
                slider1(fib_in: &f64) -> [fib_in_slider] = ret![*fib_in];
                slider2(fib_in_slider: &f64) -> [fib_in] = ret![*fib_in_slider];
            }
        }
    };

    let circle = component! {
        component circle {
            let circle_a: Circle = Circle::new(0, 0, 20), circle_b: Circle = Circle::new(1, 2, 20);

            constraint no_circle_overlap {
                shift_b(circle_a: &Circle, circle_b: &Circle) -> [circle_b] = ret![circle_a.shift(circle_b)];
                shift_a(circle_b: &Circle, circle_a: &Circle) -> [circle_a] = ret![circle_b.shift(circle_a)];
            }

        }
    };

    let db = component! {
        component db {
            let db_name: String = "foo", db_age: DbResult = None, db_double_age: Option<i32> = None;
            constraint get_age {
                db_lookup(db_name: &String) -> [db_age] = {
                    let fib_res = slow_fib(40);
                    ret![match db_name.as_str() {
                        "foo" => Some(fib_res/3),
                        "bar" => Some(fib_res/2),
                        "baz" => Some(fib_res),
                        _ => None,
                    }]
                };
            }
            constraint double_age {
                double_age(db_age: &DbResult) -> [db_double_age] = {
                    ret![db_age.as_ref().map(|age| 2*age)]
                };
            }
        }
    };

    let fibs = component! {
        component fibs {
            let chain1: f64 = 40, chain2: f64 = 0, chain3: f64 = 0, chain4: f64 = 0, chain4p2: f64 = 0, chain5: f64 = 0;
            constraint one {
                one(chain1: &f64) -> [chain2] = {
                    ret![slow_fib(*chain1 as i32) as f64]
                };
            }
            constraint two {
                two(chain1: &f64) -> [chain3] = {
                    ret![slow_fib(*chain1 as i32) as f64]
                };
            }
            constraint three {
                three(chain1: &f64) -> [chain4] = {
                    // ret![slow_fib(*chain1 as i32) as f64]
                    match *chain1 {
                        n if 37.5 < n && n < 38.5 => ret![],
                        n if 38.5 < n && n < 39.5 => fail!("Custom error, value was {}", n),
                        n if 40.5 < n && n < 41.5 => ret![0.3, 0.8],
                        _ => ret![slow_fib(*chain1 as i32) as f64]
                    }
                };
            }
            constraint three_p2 {
                eq(chain4: &f64) -> [chain4p2] = ret![*chain4];
            }
            constraint four {
                three(chain1: &f64) -> [chain5] = {
                    ret![slow_fib(*chain1 as i32) as f64]
                };
            }
        }
    };

    // Add the component to the constraint system
    let mut cs = ConstraintSystem::new();
    cs.add_component(arithmetic);
    cs.add_component(concat);
    cs.add_component(fib);
    cs.add_component(circle);
    cs.add_component(db);
    cs.add_component(fibs);
    cs
}

/// Wraps a [`ConstraintSystem<MyValue>`] so that it can be used from JavaScript.
#[wasm_bindgen]
pub fn demo_cs() -> JsConstraintSystem {
    JsConstraintSystem::wrap(demo_cs_inner()).expect("Could not create JsConstraintSystem")
}
