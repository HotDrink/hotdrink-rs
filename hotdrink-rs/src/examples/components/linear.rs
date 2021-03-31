//! A function for creating linear constraint systems.

use super::factory::{cname, vname, ComponentFactory};
use crate::{
    macros::{RawComponent, RawConstraint, RawMethod},
    Component,
};
use std::{fmt::Debug, sync::Arc};

struct LinearOneway;

impl ComponentFactory for LinearOneway {
    fn build_component<S, T>(name: S, n_constraints: usize) -> Component<T>
    where
        S: Into<String>,
        T: Clone + Debug + Default + 'static,
    {
        let n_variables = n_constraints + 1;

        // Shared dummy apply function
        let apply = Arc::new(Ok);

        // Shared variable names
        let variable_names: Vec<String> = (0..n_variables).map(vname).collect();

        // Create constraints between each consecutive variable
        let mut constraints = Vec::new();
        for constraint_id in 1..n_variables {
            let prev = &vname(constraint_id - 1);
            let current = &vname(constraint_id);
            let constraint: RawConstraint<T> = RawConstraint::new(
                &cname(constraint_id),
                vec![RawMethod::new(
                    "right",
                    vec![prev],
                    vec![current],
                    apply.clone(),
                )],
            );
            constraints.push(constraint);
        }

        // Construct component
        let name = name.into();
        let comp = RawComponent::new(
            name,
            variable_names,
            vec![T::default(); n_variables],
            constraints,
        );

        comp.into_component()
    }
}

struct LinearTwoway;

impl ComponentFactory for LinearTwoway {
    fn build_component<S, T>(name: S, n_constraints: usize) -> Component<T>
    where
        S: Into<String>,
        T: Clone + Debug + Default + 'static,
    {
        let n_variables = n_constraints + 1;

        // Shared dummy apply function
        let apply = Arc::new(Ok);

        // Shared variable names
        let variable_names: Vec<String> = (0..n_variables).map(vname).collect();

        // Create constraints between each consecutive variable
        let mut constraints = Vec::new();
        for constraint_id in 1..n_variables {
            let prev = &vname(constraint_id - 1);
            let current = &vname(constraint_id);
            let constraint: RawConstraint<T> = RawConstraint::new(
                &cname(constraint_id),
                vec![
                    RawMethod::new("left", vec![prev], vec![current], apply.clone()),
                    RawMethod::new("right", vec![current], vec![prev], apply.clone()),
                ],
            );
            constraints.push(constraint);
        }

        // Construct component
        let name = name.into();
        let comp = RawComponent::new(
            name,
            variable_names,
            vec![T::default(); n_variables],
            constraints,
        );

        comp.into_component()
    }
}
