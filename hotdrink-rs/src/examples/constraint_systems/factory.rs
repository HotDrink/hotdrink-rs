//! A generic function for constructing [`ConstraintSystem`]s.

use crate::{
    macros::{RawComponent, RawConstraint},
    model::{Component, ConstraintSystem},
};
use std::fmt::Debug;

/// Construct a constraint system with the specified number of components and variables,
/// and creates constraints using the the provided function.
pub fn make_component<T>(
    n_variables: usize,
    make_constraints: impl Fn(&[String], &[String]) -> Vec<RawConstraint<T>>,
) -> Component<T>
where
    T: Debug + Clone + Default + 'static,
{
    // Shared variable names
    let variable_names: Vec<String> = (0..n_variables)
        .map(|id| "var".to_string() + &id.to_string())
        .collect();
    let constraint_names: Vec<String> = (0..n_variables)
        .map(|id| "constraint".to_string() + &id.to_string())
        .collect();

    // Create constraints between each consecutive variable
    let constraints: Vec<RawConstraint<T>> = make_constraints(&constraint_names, &variable_names);

    // Construct component
    let name = "0".to_string();
    let comp = RawComponent::new(
        name,
        variable_names,
        vec![T::default(); n_variables],
        constraints,
    );

    comp.into_component()
}

/// Construct a constraint system with the specified number of components and variables,
/// and creates constraints using the the provided function.
pub fn make_cs<T>(
    _: usize,
    n_variables: usize,
    make_constraints: impl Fn(&[String], &[String]) -> Vec<RawConstraint<T>>,
) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    let component = make_component(n_variables, make_constraints);
    let mut cs = ConstraintSystem::new();
    cs.add_component(component);
    cs
}
