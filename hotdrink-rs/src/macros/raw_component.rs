//! Types and operations for creating [`RawComponent`]s that are easier to make manually than [`Component`].
//! They can then be converted to real [`Component`]s later.
//!
//! [`Component`]: crate::Component

use super::raw_constraint::RawConstraint;
use crate::data::component::Component;
use std::{collections::HashMap, fmt::Debug};

/// An intermediate struct for constructing [`Component`]s.
#[derive(PartialEq, Debug)]
pub struct RawComponent<'a, T> {
    name: &'a str,
    variables: Vec<&'a str>,
    values: Vec<T>,
    constraints: Vec<RawConstraint<'a, T>>,
}

impl<'a, T> RawComponent<'a, T> {
    /// Constructs a new [`RawComponent`].
    pub fn new(
        name: &'a str,
        variables: Vec<&'a str>,
        values: Vec<T>,
        constraints: Vec<RawConstraint<'a, T>>,
    ) -> Self {
        Self {
            name,
            variables,
            values,
            constraints,
        }
    }

    /// Get a map from variable name to its index
    pub fn indices(&self) -> HashMap<String, usize> {
        let mut name_to_index = HashMap::new();
        for (i, v) in self.variables.iter().enumerate() {
            name_to_index.insert(v.to_string(), i);
        }
        name_to_index
    }

    /// Add a new constraint to the component.
    pub fn add_constraint(&mut self, c: RawConstraint<'a, T>) {
        self.constraints.push(c);
    }

    /// Converts this [`RawComponent`] into a [`Component`].
    pub fn into_component(self) -> Component<T>
    where
        T: Clone,
    {
        let var_to_idx = self.indices();
        // Create constraints
        let constraints = self
            .constraints
            .into_iter()
            .map(|c| c.into_constraint(&var_to_idx))
            .collect();

        // Combine into component
        Component::new_with_map(
            self.name.to_string(),
            var_to_idx.into_iter().map(|(k, v)| (k, v)).collect(),
            self.values,
            constraints,
        )
    }
}
