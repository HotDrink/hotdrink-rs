//! A function for creating linear constraint systems.

use super::factory::{cname, vname, ComponentFactory};
use crate::{
    macros::{RawComponent, RawConstraint, RawMethod},
    Component,
};
use std::{fmt::Debug, sync::Arc};

/// A component factory for creating linear-oneway components.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LinearOneway;

impl ComponentFactory for LinearOneway {
    fn build<T>(n_constraints: usize) -> Component<T>
    where
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
        let comp = RawComponent::new(
            "linear-oneway".to_string(),
            variable_names,
            vec![T::default(); n_variables],
            constraints,
        );

        comp.into_component()
    }
}

/// A component factory for creating linear-twoway components.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LinearTwoway;

impl ComponentFactory for LinearTwoway {
    fn build<T>(n_constraints: usize) -> Component<T>
    where
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
        let comp = RawComponent::new(
            "linear-twoway".to_string(),
            variable_names,
            vec![T::default(); n_variables],
            constraints,
        );

        comp.into_component()
    }
}

#[cfg(test)]
mod tests {
    use super::{super::factory::ComponentFactory, LinearOneway};
    use crate::{Component, ComponentSpec};

    #[test]
    fn linear_oneway_right_number_of_constraints() {
        for nc in 0..20 {
            let ladder: Component<()> = LinearOneway::build(nc);
            assert_eq!(ladder.n_constraints(), nc);
        }
    }

    #[test]
    fn linear_twoway_right_number_of_constraints() {
        for nc in 0..20 {
            let ladder: Component<()> = LinearOneway::build(nc);
            assert_eq!(ladder.n_constraints(), nc);
        }
    }
}
