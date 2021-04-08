//! A function for creating ladder-shaped constraint systems.
//! See [ladder](crate::examples::components::Ladder) for more information.

use crate::{
    examples::components::{ComponentFactory, Ladder},
    model::ConstraintSystem,
};
use std::fmt::Debug;

/// Constructs a "ladder"-shaped constraint system.
pub fn ladder<T>(n_components: usize, n_variables: usize) -> ConstraintSystem<T>
where
    T: Debug + Default + Clone + 'static,
{
    let mut cs = ConstraintSystem::new();
    for _ in 0..n_components {
        cs.add_component(Ladder::build(n_variables));
    }
    cs
}
