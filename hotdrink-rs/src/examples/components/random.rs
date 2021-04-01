//! A module for generating random components.
//! The goal is to have them approximate what the average user would need.

use super::factory::ComponentFactory;
use crate::{Component, Constraint, Method, MethodSpec};
use std::{fmt::Debug, sync::Arc};

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

/// Generate `n` random `usize`s in the specified range [`min`, `max`).
fn randoms(min: usize, max: usize, n: usize) -> Vec<usize> {
    // Fill buffer with random data
    let ratio = std::mem::size_of::<usize>() / std::mem::size_of::<u8>();
    let mut buf = vec![0; n * ratio];
    getrandom::getrandom(&mut buf).expect("Could not get random numbers");

    // Convert from Vec<u8> to Vec<usize>
    let (ptr, length, capacity) = buf.into_raw_parts();
    let ptr = ptr as *mut usize;
    let mut result: Vec<usize> =
        unsafe { Vec::from_raw_parts(ptr, length / ratio, capacity / ratio) };

    // Place in correct range
    for v in &mut result {
        *v = *v % (max - min) + min;
    }

    result
}

/// Choose a random element from the input-vector, then remove it.
fn choose<T>(v: &mut Vec<T>) -> Option<T> {
    if v.is_empty() {
        return None;
    }
    let index = random_inclusive(0, v.len() - 1)?;
    Some(v.swap_remove(index))
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
pub fn make_random<T>(n_constraints: usize, max_vars_per_constraint: usize) -> Component<T>
where
    T: Clone + Default + 'static,
{
    if max_vars_per_constraint < 2 {
        panic!("Must have two or more variables per constraint");
    }

    let mut n_variables = n_constraints;
    let mut used_variables: Vec<usize> = vec![0];
    let mut unused_variables: Vec<usize> = (1..n_variables).collect();
    let mut constraints = Vec::new();

    while constraints.len() < n_constraints {
        // If no more unused, add one
        if unused_variables.is_empty() {
            unused_variables.push(n_variables);
            n_variables += 1;
        }

        // Get one used and one unused variable
        let used = unwrap_or_break!(choose(&mut used_variables));
        let unused = unwrap_or_break!(choose(&mut unused_variables));
        used_variables.push(used);
        used_variables.push(unused);

        // The number of additional variables, 0 to max, minus 2 since we already got two.
        let n_other_variables = unwrap_or_break!(random_inclusive(0, max_vars_per_constraint - 2));
        let mut actual_variables = randoms(0, n_variables, n_other_variables);
        actual_variables.push(used);

        // Manual drain filter with swap_remove
        let mut i = 0;
        while i != unused_variables.len() {
            if actual_variables.contains(&i) {
                let removed = unused_variables.swap_remove(i);
                used_variables.push(removed);
            } else {
                i += 1;
            }
        }
        // unused_variables.drain_filter(|i| actual_variables.contains(i));

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
            let n_outputs = outputs.len();
            let method = Method::new(
                format!("m{}", i),
                inputs,
                outputs.to_vec(),
                Arc::new(move |_| Ok(vec![T::default(); n_outputs])),
            );
            methods.push(method);
        }

        // Create constraint
        let constraint = Constraint::new_with_name(format!("c{}", constraints.len()), methods);
        constraints.push(constraint);
    }

    let name_to_idx = (0..n_variables).map(|i| (format!("var{}", i), i)).collect();
    Component::new_with_map(
        "random".to_string(),
        name_to_idx,
        vec![T::default(); n_variables],
        constraints,
    )
}

/// A component factory for creating random components.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Random;

impl ComponentFactory for Random {
    fn build_component<T>(n_constraints: usize) -> Component<T>
    where
        T: Clone + Debug + Default + 'static,
    {
        make_random(n_constraints, 5)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::ComponentFactory, choose, make_random, random_inclusive, Random};
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

        for i in 1..10000 {
            let mut v = (0..i).collect();
            let x = choose(&mut v);
            assert!(x.is_some() && !v.contains(&x.unwrap()));
        }
    }

    #[test]
    fn random_is_solvable() {
        for _ in 0..10000 {
            let size = random_inclusive(0, 100).unwrap();
            let random: Component<i32> = make_random(size, 5);
            assert!(crate::algorithms::simple_planner::simple_planner(&random).is_some())
        }
    }

    #[test]
    #[ignore = "TODO: Should this work? Would be nice for benchmarks to guarantee it."]
    fn random_makes_enough_constraints() {
        use crate::ComponentSpec;
        for _ in 0..10000 {
            let size = random_inclusive(0, 100).unwrap();
            let random: Component<i32> = make_random(size, 5);
            assert_eq!(random.constraints().len(), size);
        }
    }

    #[test]
    fn randoms() {
        let random_usize = super::randoms(5, 100, 500);
        for r in random_usize {
            assert!((5..100).contains(&r));
        }
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn construct_random_bench(b: &mut Bencher) {
        const N_CONSTRAINTS: usize = 10_000;
        b.iter(|| Random::build_component::<()>(N_CONSTRAINTS));
    }
}
