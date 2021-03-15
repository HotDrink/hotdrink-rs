//! Types for a [`Constraint`], an relation between variables in a [`Component`](crate::Component) that should hold.
//!
//! A [`Constraint`] contains a number of [`Method`](crate::Method)s that read from and write to different variables,
//! and can be executed in order to enforce the constraint.

use super::method::Method;
use crate::{
    algorithms::hierarchical_planner::Vertex,
    data::traits::{ConstraintLike, MethodLike},
    macros::raw_constraint::Assert,
};
use std::{collections::HashSet, fmt::Debug, ops::Index};

/// Represents a constraint in a multiway dataflow constraint system.
/// It has a name, a set of variables it references, a set of [`Method`]s to enforce it,
/// and an optional assertion to run to ensure that it is actually enforced upon running a method.
#[derive(Clone)]
pub struct Constraint<T> {
    name: String,
    variables: Vec<usize>,
    methods: Vec<Method<T>>,
    assert: Option<Assert<T>>,
}

impl<T> Debug for Constraint<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constraint")
            .field("methods", &self.methods)
            .finish()
    }
}

impl<T: Clone> ConstraintLike for Constraint<T> {
    type Method = Method<T>;

    fn new(methods: Vec<Self::Method>) -> Self {
        Self::new_with_name(String::new(), methods)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn methods(&self) -> &[Self::Method] {
        &self.methods
    }

    fn add_method(&mut self, m: Method<T>) {
        self.methods.push(m);
        let mut set = HashSet::with_capacity(self.variables.len());
        Self::fill_variables(&mut set, &self.methods);
        self.variables = set.into_iter().collect();
    }

    /// Remove a method from the constraint system.
    ///
    /// # Panics
    ///
    /// Panics if the method name is ambiguous.
    fn remove_method(&mut self, name: &str) {
        let removed: Vec<_> = self.methods.drain_filter(|m| m.name() == name).collect();
        match removed.len() {
            0 => panic!("No method named {}", name),
            1 => {}
            _ => panic!("Ambiguous method name {}", name),
        }
        // TODO: Update instead of clearing and refilling somehow?
        let mut set = HashSet::with_capacity(self.variables.len());
        Self::fill_variables(&mut set, &self.methods);
        self.variables = set.into_iter().collect();
    }

    fn variables(&self) -> &[usize] {
        &self.variables
    }
}

impl<T> Constraint<T> {
    fn fill_variables(variables: &mut HashSet<usize>, methods: &[Method<T>]) {
        for m in methods {
            for i in m.inputs() {
                variables.insert(*i);
            }
            for o in m.outputs() {
                variables.insert(*o);
            }
        }
    }

    /// Constructs a new [`Component`](crate::Component) with the specified name.
    pub fn new_with_name(name: String, methods: Vec<Method<T>>) -> Self {
        Self::new_with_name_and_assert(name, methods, None)
    }

    /// Constructs a new [`Component`](crate::Component) with the specified name and assertion.
    pub fn new_with_name_and_assert(
        name: String,
        methods: Vec<Method<T>>,
        assert: Option<Assert<T>>,
    ) -> Self {
        let mut variables: HashSet<usize> = HashSet::new();
        Self::fill_variables(&mut variables, &methods);
        Self {
            name,
            variables: variables.into_iter().collect(),
            methods,
            assert,
        }
    }
}

impl<T> PartialEq for Constraint<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.methods == other.methods
    }
}

impl<T> Index<&str> for Constraint<T> {
    type Output = Method<T>;

    fn index(&self, index: &str) -> &Self::Output {
        for m in &self.methods {
            if m.name() == index {
                return m;
            }
        }
        panic!("No method with name {}", index);
    }
}

#[cfg(test)]
mod tests {}
