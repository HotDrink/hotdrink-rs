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
        num |= (*v as usize) << i;
    }
    Some(num % (max - min + 1) + min)
}

fn choose<T>(v: &mut Vec<T>) -> Option<T> {
    if v.is_empty() {
        return None;
    }
    let index = random_inclusive(0, v.len() - 1)?;
    Some(v.swap_remove(index))
}

fn random_distinct<T>(mut v: Vec<T>, n: usize) -> Option<Vec<T>> {
    if n >= v.len() {
        return None;
    }
    let mut result = Vec::new();
    while result.len() < n {
        let value = choose(&mut v)?;
        result.push(value);
    }
    Some(result)
}

/// Return random indices, but with a higher chance of one that is used more (or less if negative clustering).
fn random_distinct_with_distribution(
    mut v: Vec<usize>,
    n: usize,
    usage: &[usize],
    clustering: i32,
) -> Option<Vec<usize>> {
    if n >= v.len() {
        return None;
    }
    let mut result = Vec::new();
    while result.len() < n {
        let usage_of_v: Vec<usize> = v.iter().map(|&idx| usage[idx]).collect();
        let idx = random_with_distribution(&usage_of_v, clustering)?;
        result.push(v.swap_remove(idx));
    }
    Some(result)
}

/// Return a random index, but with a higher chance of one that is used more (or less if negative clustering).
fn random_with_distribution(usage: &[usize], clustering: i32) -> Option<usize> {
    let mut max_index = None;
    let mut max_value = None;
    let dist = usage
        .iter()
        .map(|&x| x as i64 * random_inclusive(0, 99).unwrap() as i64 * clustering as i64);
    for (i, v) in dist.enumerate() {
        if max_index.is_none() || Some(v) > max_value {
            max_value = Some(v);
            max_index = Some(i);
        }
    }
    max_index
}

macro_rules! unwrap_or_break {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => break,
        }
    };
}

/// Create a random component.
pub fn new_make_random<T>(
    n_constraints: usize,
    max_vars_per_constraint: usize,
    clustering: i32,
) -> Component<T>
where
    T: Clone + Default + 'static,
{
    if max_vars_per_constraint < 2 {
        panic!("Must have two or more variables per constraint");
    }

    let n_variables = n_constraints * max_vars_per_constraint;
    let mut variable_usage = vec![0; n_variables];
    let mut used_variables: HashSet<usize> = std::iter::once(0).collect();
    let mut unused_variables: HashSet<usize> = (0..n_variables).collect();
    let mut constraints = Vec::new();

    while constraints.len() < n_constraints {
        // Get used and unused as owned vectors
        let mut used_variables_vec = used_variables.iter().collect();
        let mut unused_variables_vec = unused_variables.iter().collect();

        // Get one used and one unused variable
        let used = *unwrap_or_break!(choose(&mut used_variables_vec));
        let unused = *unwrap_or_break!(choose(&mut unused_variables_vec));
        used_variables.insert(unused);
        unused_variables.remove(&unused);
        variable_usage[unused] += 1;

        // The number of additional variables, 0 to max, minus 2 since we already got two.
        let n_other_variables = unwrap_or_break!(random_inclusive(0, max_vars_per_constraint - 2));
        // Select the other variables with bias according to `clustering`
        let mut actual_variables = unwrap_or_break!(random_distinct_with_distribution(
            (0..n_variables).collect(),
            max_vars_per_constraint,
            &variable_usage,
            clustering,
        ));
        actual_variables.push(used);

        // Set them to used
        for &v in &actual_variables {
            variable_usage[v] += 1;
            used_variables.insert(v);
            unused_variables.remove(&v);
        }

        // Start making methods
        let mut methods = Vec::new();

        // Write to the unused one to guarantee a free variable
        let write_to_unused = Method::new(
            "m0".to_string(),
            actual_variables
                .iter()
                .take(random_inclusive(1, n_other_variables).unwrap_or(1))
                .copied()
                .collect(),
            vec![unused],
            Arc::new(|_| Ok(vec![T::default()])),
        );
        methods.push(write_to_unused);

        let n_methods = actual_variables.len();
        // Create additional methods
        for (&output, i) in actual_variables.iter().zip(1..n_methods) {
            let outputs = vec![output];
            let mut inputs = Vec::new();
            for &input in &actual_variables {
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
            methods.push(method);
        }

        // Create constraint
        let constraint = Constraint::new_with_name(format!("c{}", constraints.len()), methods);
        constraints.push(constraint);
    }

    let name_to_idx = (0..n_variables).map(|i| (format!("v{}", i), i)).collect();
    Component::new_with_map(
        "random".to_string(),
        name_to_idx,
        vec![T::default(); n_variables],
        constraints,
    )
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
    use super::{choose, make_random, new_make_random, random_inclusive};
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

    #[test]
    fn choose_test() {
        let x: Option<i32> = choose(&mut vec![]);
        assert_eq!(x, None);

        for i in 1..100 {
            let mut v = (0..i).collect();
            let x = choose(&mut v);
            assert!(x.is_some() && !v.contains(&x.unwrap()));
        }
    }

    #[test]
    fn new_random_is_solvable() {
        for _ in 0..100 {
            for &clustering in &[-100, -50, 0, 50, 100] {
                let size = random_inclusive(0, 100).unwrap();
                let random: Component<i32> = new_make_random(size, 5, clustering);
                assert!(crate::algorithms::simple_planner::simple_planner(&random).is_some())
            }
        }
    }
}
