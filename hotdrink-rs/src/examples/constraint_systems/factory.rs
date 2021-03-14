use std::fmt::Debug;

use crate::{
    data::constraint_system::ConstraintSystem,
    macros::{RawComponent, RawConstraint},
};

pub fn make_cs<T>(
    n_components: usize,
    n_variables: usize,
    make_constraints: impl for<'a> Fn(&'a [String], &'a [String]) -> Vec<RawConstraint<'a, T>>,
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

    for comp_id in 0..n_components {
        // Create constraints between each consecutive variable
        let constraints: Vec<RawConstraint<T>> =
            make_constraints(&constraint_names, &variable_names);

        // Construct component
        let name = comp_id.to_string();
        let comp = RawComponent::new(
            &name,
            variable_names.iter().map(|s| s.as_str()).collect(),
            vec![T::default(); n_variables],
            constraints,
        );

        cs.add_component(comp.into_component());
    }

    cs
}
