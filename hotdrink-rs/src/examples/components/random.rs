//! A module for generating random components.
//! The goal is to have them approximate what the average user would need.

use crate::{Component, Constraint, Method, MethodSpec};
use std::{collections::HashSet, sync::Arc};

fn get_random_usize(min: usize, max: usize) -> Option<usize> {
    if max == 0 || min >= max {
        return None;
    }

    let mut buf = [0u8; std::mem::size_of::<usize>()];
    getrandom::getrandom(&mut buf).ok()?;
    let mut num: usize = 0;
    for (i, v) in buf.iter().enumerate() {
        num |= (*v as usize) << (i as usize);
    }
    Some((num % max).overflowing_add(min).0)
}

fn get_random_distinct_usizes(min: usize, max: usize, n: usize) -> Option<Vec<usize>> {
    if max - min < n {
        return None;
    }
    let mut random_usizes = Vec::new();
    while random_usizes.len() < n {
        let random = get_random_usize(min, max)?;
        if !random_usizes.contains(&random) {
            random_usizes.push(random);
        }
    }
    Some(random_usizes)
}

/// Create a random component.
pub fn make_random<T>(
    n_constraints: usize,
    n_variables: usize,
    n_vars_per_constraint: usize,
) -> Component<T>
where
    T: Clone + Default + 'static,
{
    let mut free_variables: HashSet<usize> = (0..n_variables).collect();

    let mut constraints = Vec::new();

    let mut constraints_added = 0;
    while constraints_added < n_constraints {
        log::debug!("Adding constraint");

        // Get some random variables
        let mut variables =
            get_random_distinct_usizes(0, n_variables, n_vars_per_constraint).unwrap();
        log::debug!("Selecting {:?}", variables);

        // If all have been used before, get a guaranteed free one
        if !variables.iter().any(|v| free_variables.contains(v)) {
            let free_variable = free_variables.iter().next();
            if let Some(free_variable) = free_variable {
                log::debug!("All were used, replacing one with {}", free_variable);
                variables.pop();
                variables.push(*free_variable);
            } else {
                log::debug!("No more free variables");
                break;
            }
        }

        let mut free_idx = None;
        // Remove variables that are now used, but store index of a free variable
        for v in &variables {
            if free_variables.contains(v) {
                free_idx = Some(*v);
            }
            free_variables.remove(v);
        }
        let free_idx = free_idx.expect("One must have been free, or loop should have broken");

        let n_methods = get_random_usize(1, n_vars_per_constraint).unwrap();
        let mut methods = Vec::new();

        log::debug!("Adding {} methods", n_methods);

        // Add a method that writes to the guaranteed free variable
        {
            let outputs = vec![free_idx];
            let mut inputs = Vec::new();
            for &input in &variables {
                if input != free_idx {
                    inputs.push(input);
                }
            }
            let method = Method::new(
                format!("m{}", 0),
                inputs,
                outputs,
                Arc::new(|_| Ok(vec![T::default()])),
            );
            log::debug!("Adding {:?}", method);
            methods.push(method);
        }

        // Create n methods.
        // Start by writing to a free variable.
        for (&output, i) in variables.iter().rev().zip(1..n_methods) {
            // Skip free variable, we already wrote to it
            if output == free_idx {
                continue;
            }
            let outputs = vec![output];
            let mut inputs = Vec::new();
            for &input in &variables {
                if input != output {
                    inputs.push(input);
                }
            }
            let method = Method::new(
                format!("m{}", i),
                inputs,
                outputs,
                Arc::new(|_| Ok(vec![T::default()])),
            );
            log::debug!("Adding {:?}", method);
            methods.push(method);
        }

        constraints.push(Constraint::new_with_name(
            format!("c{}", constraints_added),
            methods,
        ));

        constraints_added += 1;
    }

    let name_to_idx = (0..n_variables).map(|i| (format!("v{}", i), i)).collect();

    Component::new_with_map(
        "random".to_string(),
        name_to_idx,
        vec![T::default(); n_variables],
        constraints,
    )
}

#[cfg(test)]
mod tests {
    use super::{get_random_usize, make_random};
    use crate::Component;

    #[test]
    fn generate_random_zero() {
        let random = get_random_usize(0, 1).unwrap();
        assert_eq!(random, 0);
    }

    #[test]
    fn generate_random_5_to_10() {
        for _ in 0..1000 {
            let min = 0;
            let max = 10;
            let random = get_random_usize(min, max);
            assert!(Some(min) <= random && random < Some(max));
        }
    }

    #[test]
    fn make_random_small_is_solvable() {
        let size = 10;
        let random: Component<i32> = make_random(size, size, 3);
        assert!(crate::algorithms::simple_planner::simple_planner(&random).is_some())
    }

    #[test]
    fn make_random_medium_is_solvable() {
        let size = 500;
        let random: Component<i32> = make_random(size, size, 5);
        assert!(crate::algorithms::simple_planner::simple_planner(&random).is_some())
    }

    #[test]
    fn make_random_big_is_solvable() {
        let size = 5000;
        let random: Component<i32> = make_random(size, size, 7);
        assert!(crate::algorithms::simple_planner::simple_planner(&random).is_some())
    }
}
