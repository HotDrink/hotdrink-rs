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
    pub fn variable<S: Into<String>>(mut self, name: S, value: T) -> Self {
        self.variables
            .insert(name.into(), Value::Ref(Arc::new(value)));
        self
    }

    /// Adds a mutable variable.
    pub fn variable_mut<S: Into<String>>(mut self, name: S, value: T) -> Self {
        self.variables
            .insert(name.into(), Value::MutRef(Arc::new(RwLock::new(value))));
        self
    }

    /// Adds immutable variables.
    pub fn variables<S: Into<String>>(mut self, variables: Vec<(S, T)>) -> Self {
        for (name, value) in variables {
            self = self.variable(name, value);
        }
        self
    }

    /// Adds mutable variables.
    pub fn variables_mut<S: Into<String>>(mut self, variables: Vec<(S, T)>) -> Self {
        for (name, value) in variables {
            self = self.variable_mut(name, value);
        }
        self
    }

    /// Adds a constraint.
    pub fn constraint(mut self, constraint: ConstraintBuilder<T>) -> Self {
        self.constraints.push(constraint);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentBuilder;
    use crate::builders::ConstraintBuilder;

    #[test]
    fn builder_builds() {
        let _: ComponentBuilder<i32> = ComponentBuilder::new("Component")
            .variables(vec![("a", 3), ("b", 7)])
            .variable_mut("c", 10)
            .constraint(
                ConstraintBuilder::new("Sum")
                    .method(method!(
                        fn m1(a: &i32) -> [b] {
                            Ok(vec![*a])
                        }
                    ))
                    .method(method!(
                        fn m2(b: &mut i32) -> [a] {
                            Ok(vec![*b])
                        }
                    )),
            )
            .constraint(ConstraintBuilder::new("Product"));
    }
}
