//! A generic function for constructing [`ConstraintSystem`]s.

use std::fmt::Debug;

use crate::{
    data::constraint_system::ConstraintSystem,
    macros::{RawComponent, RawConstraint},
};

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
    // Shared variable names
    let variable_names: Vec<String> = (0..n_variables)
        .map(|id| "var".to_string() + &id.to_string())
        .collect();
    let constraint_names: Vec<String> = (0..n_variables)
        .map(|id| "constraint".to_string() + &id.to_string())
        .collect();

    let mut cs = ConstraintSystem::new();

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

    cs.add_component(comp.into_component());

    cs
}
