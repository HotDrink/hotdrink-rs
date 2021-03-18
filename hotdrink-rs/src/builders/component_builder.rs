//! A builder-struct for programmatically creating components.

use super::{constraint_builder::ConstraintBuilder, value_experiments::Value};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// A builder for making programmatic construction of components easier.
#[derive(Clone, Debug)]
pub struct ComponentBuilder<T> {
    name: String,
    variables: HashMap<String, Value<T>>,
    constraints: Vec<ConstraintBuilder<T>>,
}

impl<T> ComponentBuilder<T> {
    /// Constructs a new `ComponentBuilder` with no variables or constraints.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            variables: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    /// Adds an immutable variable.
    pub fn variable<S: Into<String>>(&mut self, name: S, value: T) -> &mut Self {
        self.variables
            .insert(name.into(), Value::Ref(Arc::new(value)));
        self
    }

    /// Adds a mutable variable.
    pub fn variable_mut<S: Into<String>>(&mut self, name: S, value: T) -> &mut Self {
        self.variables
            .insert(name.into(), Value::MutRef(Arc::new(RwLock::new(value))));
        self
    }

    /// Adds immutable variables.
    pub fn variables<S: Into<String>>(&mut self, variables: Vec<(S, T)>) -> &mut Self {
        variables.into_iter().for_each(|(name, value)| {
            self.variable(name, value);
        });
        self
    }

    /// Adds mutable variables.
    pub fn variables_mut<S: Into<String>>(&mut self, variables: Vec<(S, T)>) -> &mut Self {
        variables.into_iter().for_each(|(name, value)| {
            self.variable_mut(name, value);
        });
        self
    }

    /// Adds a constraint.
    pub fn constraint(&mut self, constraint: ConstraintBuilder<T>) -> &mut Self {
        self.constraints.push(constraint);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentBuilder;
    use crate::builders::constraint_builder::ConstraintBuilder;
    use crate::method;

    #[test]
    fn build() {
        let _: &mut ComponentBuilder<i32> = ComponentBuilder::new("Component")
            .variables(vec![("a", 3), ("b", 7)])
            .variable_mut("c", 10)
            .constraint(
                ConstraintBuilder::new("Sum")
                    .method(method!(m1(a: &i32) -> [a] { Ok(vec![*a]) }))
                    .method(method!(m2(a: &i32) -> [a] { Ok(vec![*a]) }))
                    .method(method!(m3(a: &i32) -> [a] { Ok(vec![*a]) })),
            )
            .constraint(ConstraintBuilder::new("Product"));
    }
}
