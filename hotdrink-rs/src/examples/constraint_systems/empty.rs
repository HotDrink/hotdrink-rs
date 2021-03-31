//! A function for creating empty constraint systems.

use crate::data::constraint_system::ConstraintSystem;
use crate::macros::raw_component::RawComponent;
use std::fmt::Debug;

/// Constructs a constraint system with components and variables,
/// but no actual constraints or methods.
/// This lets us create a baseline for how well the planner and solver perform.
pub fn make_empty_cs<T>(n_components: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Clone + Default + Send + 'static,
{
    // Add the component to the constraint system
    let mut cs = ConstraintSystem::new();

    // Duplicate the component
    for comp_id in 0..n_components {
        // Define variables for the component
        let variables: Vec<String> = (0..n_variables)
            .map(|n| "var".to_string() + &n.to_string())
            .collect();

        // Construct the component
        let comp_name = comp_id.to_string();
        let raw_comp = RawComponent::new(
            comp_name,
            variables,
            vec![T::default(); n_variables],
            vec![],
        );

        // Convert and add the component
        let comp = raw_comp.into_component();
        cs.add_component(comp);
    }

    cs
}
