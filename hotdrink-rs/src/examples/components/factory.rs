//! A trait for structs that can build components.

use crate::Component;
use std::fmt::Debug;

/// Convert a variable id to a name.
/// Convenient for making consistent naming for variables
/// and constraints when automatically generating them.
pub fn vname(vid: usize) -> String {
    format!("var{}", vid)
}

/// Convert a constraint id to a name.
/// Convenient for making consistent naming for variables
/// and constraints when automatically generating them.
pub fn cname(cid: usize) -> String {
    format!("c{}", cid)
}

/// A trait for structs that can build components of a specified size.
/// This is used for creating components in benchmarks.
pub trait ComponentFactory {
    /// Returns the name of this factory.
    fn name() -> &'static str;
    /// Build the component with the specified name and number of constraints.
    fn build<T>(n_constraints: usize) -> Component<T>
    where
        T: Clone + Debug + Default + 'static;
}
