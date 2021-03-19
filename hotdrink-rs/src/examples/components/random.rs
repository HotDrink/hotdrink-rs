//! A module for generating random components.
//! The goal is to have them approximate what the average user would need.

use crate::{Component, Constraint, Method, MethodSpec};
use std::{collections::HashSet, sync::Arc};

fn random_inclusive(min: usize, max: usize) -> Option<usize> {
    if min > max {
        return None;
    }

    let mut buf = [0u8; std::mem::size_of::<usize>()];
    getrandom::getrandom(&mut buf).ok()?;
    let mut num: usize = 0;
    for (i, v) in buf.iter().enumerate() {
        num |= (*v as usize) << (i as usize);
    }
    Some(num % (max - min + 1) + min)
}

fn random_distinct<T>(mut v: Vec<T>, n: usize) -> Option<Vec<T>> {
    if n >= v.len() {
        return None;
    }
    let mut result = Vec::new();
    while result.len() < n {
        let index = random_inclusive(0, v.len() - 1)?;
        result.push(v.swap_remove(index));
    }
    Some(result)
}

/// Create a random component.
pub fn make_random<T>(n_constraints: usize, n_variables: usize) -> Component<T>
where
    T: Clone + Default + 'static,
{
    let n_vars_per_constraint = n_variables.min(5);

    // The variables that are not yet involved in a constraint
    let mut free_variables: HashSet<usize> = (0..n_variables).collect();
    // The constraints added so far
    let mut constraints = Vec::new();

    // Add constraints until done
    let mut constraints_added = 0;
    while constraints_added < n_constraints {
        log::debug!("Adding constraint");

        // How many variables should this constraint contain?
        // We want at least one variable, anything less does not make sense.
        // One makes sense if it is a constant value.
        let actual_n_variables = random_inclusive(1, n_vars_per_constraint).unwrap();
        // Get the random indices we want to include in the constraint.
        let mut variables = random_distinct(
            (0..n_variables).collect(),
            actual_n_variables.min(n_variables) - 1,
        )
        .unwrap();

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

        // Figure out how many methods to create
        let n_methods = random_inclusive(1, actual_n_variables).unwrap();
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
        for (&output, i) in variables.iter().zip(1..n_methods) {
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
    use super::{make_random, random_inclusive};
    use crate::Component;

    #[test]
    fn generate_random_zero() {
        let random = random_inclusive(0, 0).unwrap();
        assert_eq!(random, 0);
    }

    #[test]
    fn generate_randoms() {
        for max in 0..1000 {
            let min = max / 2;
            let random = random_inclusive(min, max);
            assert!(
                Some(min) <= random && random <= Some(max),
                "{:?} should have been between {} and {}",
                random,
                min,
                max
            );
        }
    }

    #[test]
    fn random_is_solvable() {
        for _ in 0..1000 {
            let size = random_inclusive(0, 100).unwrap();
            let random: Component<i32> = make_random(size, size);
            assert!(crate::algorithms::simple_planner::simple_planner(&random).is_some())
        }
    }
}
