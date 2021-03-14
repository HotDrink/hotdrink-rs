use super::raw_method::RawMethod;
use crate::data::constraint::Constraint;
use std::{collections::HashMap, fmt::Debug, sync::Arc};

/// A type for an assertion statement for the constraint.
/// This may be run after each method call to ensure that the
/// constraint holds, and may also work as documentation.
pub type Assert<T> = Arc<dyn Fn(&[T]) -> bool>;

pub struct RawConstraint<'a, T> {
    name: &'a str,
    methods: Vec<RawMethod<'a, T>>,
    assert: Option<Assert<T>>,
}

impl<'a, T> RawConstraint<'a, T> {
    pub fn new(name: &'a str, methods: Vec<RawMethod<'a, T>>) -> Self {
        Self {
            name,
            methods,
            assert: None,
        }
    }

    pub fn new_with_assert(
        name: &'a str,
        methods: Vec<RawMethod<'a, T>>,
        assert: Option<Assert<T>>,
    ) -> Self {
        Self {
            name,
            methods,
            assert,
        }
    }

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

impl<T> Debug for RawConstraint<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawConstraint")
            .field("name", &self.name)
            .field("methods", &self.methods)
            .finish()
    }
}

impl<T> PartialEq for RawConstraint<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        (self.name, &self.methods) == (other.name, &other.methods)
    }
}
