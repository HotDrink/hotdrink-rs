//! A trait for structs that can build components.

use crate::{Component, ConstraintSystem};
use std::fmt::Debug;

/// A trait for structs that can build components.
/// This is used for creating components in benchmarks.
pub trait ComponentFactory {
    /// Build the component with the specified name and number of constraints.
    fn build_component<S, T>(name: S, n_constraints: usize) -> Component<T>
    where
        S: Into<String>,
        T: Clone + Debug + Default + 'static;

    /// Use `build_component` to build a constraint system that wraps the component.
    fn build_constraint_system<S, T>(name: S, n_constraints: usize) -> ConstraintSystem<T>
    where
        S: Into<String>,
        T: Clone + Debug + Default + 'static,
    {
        let component: Component<T> = Self::build_component(name, n_constraints);
        let mut cs = ConstraintSystem::new();
        cs.add_component(component);
        cs
    }
}
