use std::{fmt::Debug, sync::Arc};

use crate::{
    data::constraint_system::ConstraintSystem,
    macros::{RawComponent, RawConstraint, RawMethod},
};

/// Constructs a constraint system with many constraints between variables.
pub fn make_dense_cs<T>(n_components: usize, n_variables: usize) -> ConstraintSystem<T>
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

    for comp_id in 0..n_components {
        let mut constraints = Vec::new();

        const STEP_SIZE: usize = 4;
        for from_idx in (0..n_variables.saturating_sub(STEP_SIZE)).step_by(STEP_SIZE) {
            for offset in 1..=4 {
                let from = &variable_names[from_idx];
                let to = &variable_names[from_idx + offset];
                let constraint: RawConstraint<'_, T> = RawConstraint::new(
                    &constraint_names[from_idx + offset],
                    vec![
                        RawMethod::new("right", vec![from], vec![to], apply.clone()),
                        RawMethod::new("left", vec![to], vec![from], apply.clone()),
                    ],
                );
                constraints.push(constraint);
            }
        }

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
