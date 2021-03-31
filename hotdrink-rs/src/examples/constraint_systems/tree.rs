//! Functions for creating tree-like constraint systems.

use std::{fmt::Debug, sync::Arc};

use super::factory::make_cs;
use crate::{
    data::constraint_system::ConstraintSystem,
    macros::{RawConstraint, RawMethod},
};

/// Root has two constraints, one to each of its children.
pub fn singleoutput_singleway<T>(n_components: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    let make_constraints: fn(&[String], &[String]) -> Vec<RawConstraint<T>> =
        |constraint_names, variable_names| {
            let n_variables = variable_names.len();
            let apply = Arc::new(Ok);
            let mut constraints = Vec::new();
            for current in 0..n_variables {
                let current_name = &variable_names[current];
                let left = 2 * current + 1;
                let right = 2 * current + 2;
                if left >= n_variables || right >= n_variables {
                    break;
                }
                let left_name = &variable_names[left];
                let right_name = &variable_names[right];
                constraints.push(RawConstraint::new(
                    &constraint_names[left],
                    vec![RawMethod::new(
                        "left",
                        vec![current_name.as_str()],
                        vec![left_name.as_str()],
                        apply.clone(),
                    )],
                ));
                constraints.push(RawConstraint::new(
                    &constraint_names[right],
                    vec![RawMethod::new(
                        "right",
                        vec![current_name.as_str()],
                        vec![right_name.as_str()],
                        apply.clone(),
                    )],
                ));
            }
            constraints
        };
    make_cs(n_components, n_variables, make_constraints)
}

/// Root has two constraints, one to each of its children.
/// Each child has a method back to the root.
pub fn singleoutput_multiway<T>(n_components: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    let make_constraints: fn(&[String], &[String]) -> Vec<RawConstraint<T>> =
        |constraint_names, variable_names| {
            let n_variables = variable_names.len();
            let apply = Arc::new(Ok);
            let mut constraints = Vec::new();
            for current in 0..n_variables {
                let current_name = &variable_names[current];
                let left = 2 * current + 1;
                let right = 2 * current + 2;
                if left >= n_variables || right >= n_variables {
                    break;
                }
                let left_name = &variable_names[left];
                let right_name = &variable_names[right];
                constraints.push(RawConstraint::new(
                    &constraint_names[left],
                    vec![
                        RawMethod::new(
                            "left1",
                            vec![current_name.as_str()],
                            vec![left_name.as_str()],
                            apply.clone(),
                        ),
                        RawMethod::new(
                            "left2",
                            vec![left_name.as_str()],
                            vec![current_name.as_str()],
                            apply.clone(),
                        ),
                    ],
                ));
                constraints.push(RawConstraint::new(
                    &constraint_names[right],
                    vec![
                        RawMethod::new(
                            "right1",
                            vec![current_name.as_str()],
                            vec![right_name.as_str()],
                            apply.clone(),
                        ),
                        RawMethod::new(
                            "right2",
                            vec![right_name.as_str()],
                            vec![current_name.as_str()],
                            apply.clone(),
                        ),
                    ],
                ));
            }
            constraints
        };
    make_cs(n_components, n_variables, make_constraints)
}

/// The root has one constraint with its children, and one method that outputs to both.
pub fn multioutput_singleway<T>(n_components: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    let make_constraints: fn(&[String], &[String]) -> Vec<RawConstraint<T>> =
        |constraint_names, variable_names| {
            let n_variables = variable_names.len();
            let apply = Arc::new(|v: Vec<T>| {
                let value = v[0].clone();
                Ok(vec![value.clone(), value])
            });
            let mut constraints = Vec::new();
            for current in 0..n_variables {
                let current_name = &variable_names[current];
                let left = 2 * current + 1;
                let right = 2 * current + 2;
                if left >= n_variables || right >= n_variables {
                    break;
                }
                let left_name = &variable_names[left];
                let right_name = &variable_names[right];
                constraints.push(RawConstraint::new(
                    &constraint_names[current],
                    vec![RawMethod::new(
                        "m",
                        vec![current_name.as_str()],
                        vec![left_name.as_str(), right_name.as_str()],
                        apply.clone(),
                    )],
                ));
            }
            constraints
        };
    make_cs(n_components, n_variables, make_constraints)
}

/// The root has one constraint with its children, one method that outputs to both,
/// and the left child has one that writes to the root and right child.
pub fn multioutput_twoway<T>(n_components: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    let make_constraints: fn(&[String], &[String]) -> Vec<RawConstraint<T>> =
        |constraint_names, variable_names| {
            let n_variables = variable_names.len();
            let apply = Arc::new(|v: Vec<T>| {
                let value = v[0].clone();
                Ok(vec![value.clone(), value])
            });
            let mut constraints = Vec::new();
            for current in 0..n_variables {
                let current_name = &variable_names[current];
                let left = 2 * current + 1;
                let right = 2 * current + 2;
                if left >= n_variables || right >= n_variables {
                    break;
                }
                let left_name = &variable_names[left];
                let right_name = &variable_names[right];
                constraints.push(RawConstraint::new(
                    &constraint_names[current],
                    vec![
                        RawMethod::new(
                            "top_to_left_right",
                            vec![current_name.as_str()],
                            vec![left_name.as_str(), right_name.as_str()],
                            apply.clone(),
                        ),
                        RawMethod::new(
                            "left_to_top_right",
                            vec![left_name.as_str()],
                            vec![current_name.as_str(), right_name.as_str()],
                            apply.clone(),
                        ),
                    ],
                ));
            }
            constraints
        };
    make_cs(n_components, n_variables, make_constraints)
}

/// The root has one constraint with its children, there are three methods,
/// each one reads from one of them and writes to the two others.
pub fn multioutput_threeway<T>(n_components: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    let make_constraints: fn(&[String], &[String]) -> Vec<RawConstraint<T>> =
        |constraint_names, variable_names| {
            let n_variables = variable_names.len();
            let apply = Arc::new(|v: Vec<T>| {
                let value = v[0].clone();
                Ok(vec![value.clone(), value])
            });
            let mut constraints = Vec::new();
            for current in 0..n_variables {
                let current_name = &variable_names[current];
                let left = 2 * current + 1;
                let right = 2 * current + 2;
                if left >= n_variables || right >= n_variables {
                    break;
                }
                let left_name = &variable_names[left];
                let right_name = &variable_names[right];
                constraints.push(RawConstraint::new(
                    &constraint_names[current],
                    vec![
                        RawMethod::new(
                            "top_to_left_right",
                            vec![current_name.as_str()],
                            vec![left_name.as_str(), right_name.as_str()],
                            apply.clone(),
                        ),
                        RawMethod::new(
                            "left_to_top_right",
                            vec![left_name.as_str()],
                            vec![current_name.as_str(), right_name.as_str()],
                            apply.clone(),
                        ),
                        RawMethod::new(
                            "right_to_top_left",
                            vec![right_name.as_str()],
                            vec![current_name.as_str(), left_name.as_str()],
                            apply.clone(),
                        ),
                    ],
                ));
            }
            constraints
        };
    make_cs(n_components, n_variables, make_constraints)
}

/// The root has one constraint with its children, there are three methods,
/// each one reads from one of them and writes to the two others.
pub fn unprunable<T>(n_components: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    let make_constraints: fn(&[String], &[String]) -> Vec<RawConstraint<T>> =
        |constraint_names, variable_names| {
            let n_variables = variable_names.len();
            let apply = Arc::new(|v: Vec<T>| {
                let value = v[0].clone();
                Ok(vec![value.clone(), value])
            });
            let mut constraints = Vec::new();
            for current in 0..n_variables {
                let current_name = &variable_names[current];
                let left = 2 * current + 1;
                let right = 2 * current + 2;
                if left >= n_variables || right >= n_variables {
                    break;
                }
                let left_name = &variable_names[left];
                let right_name = &variable_names[right];
                constraints.push(RawConstraint::new(
                    &constraint_names[current],
                    vec![
                        RawMethod::new(
                            "down_left",
                            vec![current_name.as_str(), right_name.as_str()],
                            vec![left_name.as_str()],
                            apply.clone(),
                        ),
                        RawMethod::new(
                            "down_right",
                            vec![current_name.as_str(), left_name.as_str()],
                            vec![right_name.as_str()],
                            apply.clone(),
                        ),
                    ],
                ));
            }
            constraints
        };
    make_cs(n_components, n_variables, make_constraints)
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::unprunable;
    use crate::{
        algorithms::hierarchical_planner::hierarchical_planner, data::traits::ComponentSpec,
    };
    use test::Bencher;

    #[bench]
    fn bench_unprunable(b: &mut Bencher) {
        let cs = unprunable::<()>(1, 400);
        let comp = cs.get_component("0");
        let ranking: Vec<usize> = (0..comp.n_variables()).collect();
        b.iter(|| hierarchical_planner(comp, &ranking));
    }
}
