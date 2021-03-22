//! A module for generating random components.
//! The goal is to have them approximate what the average user would need.

use crate::{Component, Constraint, Method, MethodSpec};
use std::{
    collections::{BinaryHeap, HashSet},
    sync::Arc,
};

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

/// Choose a random element from the input-vector, then remove it.
fn choose<T>(v: &mut Vec<T>) -> Option<T> {
    if v.is_empty() {
        return None;
    }
    let index = random_inclusive(0, v.len() - 1)?;
    Some(v.swap_remove(index))
}

/// Assign a cluster to a constraint based on its index.
fn assign_cluster(constraint: usize, n_clusters: usize) -> usize {
    constraint % n_clusters
}

/// Pick random variables, but prefer ones in the cluster of the constraint.
fn randoms_with_clustering(
    values: Vec<usize>,
    n_values: usize,
    constraint: usize,
    n_clusters: usize,
    clustering_strength: f32,
) -> Option<Vec<usize>> {
    // Assign cluster to constraint
    let cluster_id = assign_cluster(constraint, n_clusters);
    let cluster_size = values.len() / n_clusters;
    // Get random value for each variable, with a bias towards the ones in the cluster
    let random_values: Vec<usize> = (0..values.len())
        .map(|i| {
            // Get random value
            let mut value = random_inclusive(0, 999).unwrap();
            // Check if it is in the constraint's cluster
            if cluster_size == 0 || i / cluster_size == cluster_id {
                // Give bias according to clustering strength
                value = (value as f32 * clustering_strength) as usize;
            }
            value
        })
        .collect();

    // Push all the values and corresponding indices to a binary heap
    // Value first to compare on that, not index.
    let mut bh = BinaryHeap::new();
    for (i, rv) in random_values.into_iter().enumerate() {
        bh.push((rv, i));
    }

    // Get the n greatest values
    let mut result = Vec::new();
    for _ in 0..n_values {
        let (_, i) = bh.pop()?;
        result.push(i);
    }

    Some(result)
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
pub fn make_random<T>(
    n_constraints: usize,
    max_vars_per_constraint: usize,
    n_clusters: usize,
    clustering: f32,
) -> Component<T>
where
    T: Clone + Default + 'static,
{
    if max_vars_per_constraint < 2 {
        panic!("Must have two or more variables per constraint");
    }

    let mut n_variables = n_constraints;
    let mut variable_usage = vec![0; n_variables];
    let mut used_variables: HashSet<usize> = std::iter::once(0).collect();
    let mut unused_variables: HashSet<usize> = (0..n_variables).collect();
    let mut constraints = Vec::new();

    while constraints.len() < n_constraints {
        // If no more unused, add one
        if unused_variables.is_empty() {
            unused_variables.insert(n_variables);
            n_variables += 1;
            variable_usage.push(0);
        }
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
        let actual_variables = randoms_with_clustering(
            (0..n_variables).collect(),
            n_other_variables,
            constraints.len(),
            n_clusters,
            clustering,
        );
        let mut actual_variables = unwrap_or_break!(actual_variables);
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

        // Does not have to write to all variables.
        let n_methods = random_inclusive(0, actual_variables.len()).unwrap();
        // May write to all of them
        let outputs_per_method = random_inclusive(1, actual_variables.len()).unwrap();

        // Create additional methods
        for (outputs, i) in actual_variables
            .chunks(outputs_per_method)
            .zip(1..n_methods)
        {
            let mut inputs = Vec::new();
            for &input in &actual_variables {
                inputs.push(input);
            }
            let method = Method::new(
                format!("m{}", i),
                inputs,
                outputs.to_vec(),
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

#[cfg(test)]
mod tests {
    use super::{choose, make_random, random_inclusive};
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
    fn random_is_solvable() {
        for _ in 0..100 {
            for &clustering in &[0.0, 0.25, 0.5, 0.75, 1.0] {
                let size = random_inclusive(0, 100).unwrap();
                let random: Component<i32> = make_random(size, 5, 5, clustering);
                assert!(crate::algorithms::simple_planner::simple_planner(&random).is_some())
            }
        }
    }

    #[test]
    // #[ignore = "TODO: Should this work? Would be nice for benchmarks to guarantee it."]
    fn random_makes_enough_constraints() {
        use crate::ComponentSpec;
        for _ in 0..100 {
            for &clustering in &[0.0, 0.25, 0.5, 0.75, 1.0] {
                let size = random_inclusive(0, 100).unwrap();
                let random: Component<i32> = make_random(size, 5, 5, clustering);
                assert_eq!(random.constraints().len(), size);
            }
        }
    }
}
