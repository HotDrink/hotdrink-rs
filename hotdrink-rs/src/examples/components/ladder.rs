//! An example component used in benchmarking.
//! It was originally defined [here](https://github.com/HotDrink/hotdrink/blob/3f9bc25751079c69f8277918521a75dd1163bee4/test/models/ladder-10.js).

use crate::{Component, Constraint, Method, MethodFailure, MethodLike};
use std::{collections::HashMap, sync::Arc};

fn avg<T: Default>(_: Vec<T>) -> Result<Vec<T>, MethodFailure> {
    // Ok(vec![(args[0] + args[1]) / 2.0])
    Ok(vec![T::default()])
}

fn rev<T: Default>(_: Vec<T>) -> Result<Vec<T>, MethodFailure> {
    // Ok(vec![2.0 * args[0] - args[1]])
    Ok(vec![T::default()])
}

/// Generate a component with the "ladder"-shape.
pub fn ladder<T>(name: String, n_variables: usize) -> Component<T>
where
    T: Clone + Default + 'static,
{
    let values = vec![T::default(); n_variables];

    let mut constraints = Vec::new();
    for i in (0..n_variables.saturating_sub(3)).step_by(2) {
        let a0 = i;
        let b0 = i + 1;
        let a1 = i + 2;
        let b1 = i + 3;
        let lower = Constraint::new_with_name(
            format!("c{}", i),
            vec![
                Method::new("lower1".to_string(), vec![a0, a1], vec![b0], Arc::new(avg)),
                Method::new("lower2".to_string(), vec![b0, a0], vec![a1], Arc::new(rev)),
                Method::new("lower3".to_string(), vec![b0, a1], vec![a0], Arc::new(rev)),
            ],
        );
        constraints.push(lower);
        let upper = Constraint::new_with_name(
            format!("c{}", i + 1),
            vec![
                Method::new("upper1".to_string(), vec![b0, b1], vec![a1], Arc::new(avg)),
                Method::new("upper2".to_string(), vec![a1, b0], vec![b1], Arc::new(rev)),
                Method::new("upper3".to_string(), vec![a1, b1], vec![b0], Arc::new(rev)),
            ],
        );
        constraints.push(upper);
    }
    let mut name_to_index = HashMap::new();
    for i in 0..n_variables {
        name_to_index.insert(format!("var{}", i), i);
    }
    Component::new_with_map(name, name_to_index, values, constraints)
}

#[cfg(test)]
mod tests {
    use super::ladder;
    use crate::data::traits::ComponentLike;

    #[test]
    fn constructs_without_error() {
        for i in 0..20 {
            let mut ladder = ladder::<()>("ladder".to_string(), i);
            let result = ladder.update();
            assert_eq!(result, Ok(()));
        }
    }
}
