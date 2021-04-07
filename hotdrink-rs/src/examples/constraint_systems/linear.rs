//! A function for creating linear constraint systems.

use std::{fmt::Debug, sync::Arc};

use crate::{
    macros::{RawComponent, RawConstraint, RawMethod},
    model::ConstraintSystem,
};

/// Constructs a constraint system with long chains of constraints between variables.
pub fn linear_oneway<T>(_: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    // Shared dummy apply function
    let apply = Arc::new(Ok);

    // Shared variable names
    let variable_names: Vec<String> = (0..n_variables)
        .map(|id| "var".to_string() + &id.to_string())
        .collect();
    let constraint_names: Vec<String> = (0..n_variables)
        .map(|id| "constraint".to_string() + &id.to_string())
        .collect();

    let mut cs = ConstraintSystem::new();

    // Create constraints between each consecutive variable
    let mut constraints = Vec::new();
    for constraint_id in 1..n_variables {
        let prev: &str = &variable_names[constraint_id - 1];
        let current = &variable_names[constraint_id];
        let constraint: RawConstraint<T> = RawConstraint::new(
            &constraint_names[constraint_id],
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
    let name = "0".to_string();
    let comp = RawComponent::new(
        name,
        variable_names,
        vec![T::default(); n_variables],
        constraints,
    );

    cs.add_component(comp.into_component());

    cs
}

/// Constructs a constraint system with long chains of constraints between variables.
pub fn linear_twoway<T>(_: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    // Shared dummy apply function
    let apply = Arc::new(Ok);

    // Shared variable names
    let variable_names: Vec<String> = (0..n_variables)
        .map(|id| "var".to_string() + &id.to_string())
        .collect();
    let constraint_names: Vec<String> = (0..n_variables)
        .map(|id| "constraint".to_string() + &id.to_string())
        .collect();

    let mut cs = ConstraintSystem::new();

    // Create constraints between each consecutive variable
    let mut constraints = Vec::new();
    for constraint_id in 1..n_variables {
        let prev: &str = &variable_names[constraint_id - 1];
        let current = &variable_names[constraint_id];
        let constraint: RawConstraint<T> = RawConstraint::new(
            &constraint_names[constraint_id],
            vec![
                RawMethod::new("left", vec![prev], vec![current], apply.clone()),
                RawMethod::new("right", vec![current], vec![prev], apply.clone()),
            ],
        );
        constraints.push(constraint);
    }

    // Construct component
    let name = "0".to_string();
    let comp = RawComponent::new(
        name,
        variable_names,
        vec![T::default(); n_variables],
        constraints,
    );

    cs.add_component(comp.into_component());

    cs
}
