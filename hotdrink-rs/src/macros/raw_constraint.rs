//! Types and operations for creating [`RawConstraint`]s that are easier to make manually than [`Constraint`].
//! They can then be converted to real [`Constraint`]s later.
//!
//! [`Constraint`]: crate::Constraint

use super::raw_method::RawMethod;
use crate::data::constraint::Constraint;
use std::{collections::HashMap, fmt::Debug, sync::Arc};

/// A type for an assertion statement for the constraint.
/// This may be run after each method call to ensure that the
/// constraint holds, and may also work as documentation.
pub type Assert<T> = Arc<dyn Fn(&[T]) -> bool>;

/// An intermediate struct for constructing [`Constraint`]s.
pub struct RawConstraint<T> {
    name: String,
    methods: Vec<RawMethod<T>>,
    assert: Option<Assert<T>>,
}

impl<T> RawConstraint<T> {
    /// Constructs a new [`RawConstraint`].
    pub fn new<S: Into<String>>(name: S, methods: Vec<RawMethod<T>>) -> Self {
        Self {
            name: name.into(),
            methods,
            assert: None,
        }
    }

    /// Constructs a new [`RawConstraint`] with an optional assert statement.
    pub fn new_with_assert<S: Into<String>>(
        name: S,
        methods: Vec<RawMethod<T>>,
        assert: Option<Assert<T>>,
    ) -> Self {
        Self {
            name: name.into(),
            methods,
            assert,
        }
    }

    /// Converts this [`RawConstraint`] into a [`Constraint`].
    pub fn into_constraint(self, var_to_idx: &HashMap<String, usize>) -> Constraint<T>
    where
        T: Clone,
    {
        Constraint::new_with_name_and_assert(
            self.name.to_owned(),
            self.methods
                .into_iter()
                .map(|m| m.into_method(var_to_idx))
                .collect(),
            self.assert,
        )
    }
}

impl<T> Debug for RawConstraint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawConstraint")
            .field("name", &self.name)
            .field("methods", &self.methods)
            .finish()
    }
}

impl<T> PartialEq for RawConstraint<T> {
    fn eq(&self, other: &Self) -> bool {
        (&self.name, &self.methods) == (&other.name, &other.methods)
    }
}
