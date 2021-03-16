//! Useful traits for constraint systems that define the interface required for planning.

use crate::algorithms::hierarchical_planner::Vertex;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    sync::Arc,
};

/// The potential errors from performing a method call.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MethodFailure {
    /// An attempt to use a variable that does not exist.
    NoSuchVariable(String),
    /// Failure to convert a variable into the specified type.
    TypeConversionFailure(String, String),
    /// The constraint satisfaction method received too few or too many values as input.
    WrongInputCount(usize, usize),
    /// The constraint satisfaction method returned too few or too many values as output.
    WrongOutputCount(usize, usize),
    /// A custom error from the programmer.
    Custom(String),
}

/// The result of calling a method's function.
pub type MethodResult<T> = Result<Vec<T>, MethodFailure>;

/// The function contained within a method.
pub type MethodFunction<T> = Arc<dyn Fn(Vec<T>) -> Result<Vec<T>, MethodFailure> + Send + Sync>;

/// An extension of the [`Vertex`] trait for methods.
pub trait MethodSpec: Vertex {
    /// The input and output type of the method.
    type Arg;
    /// Constructs a new [`MethodSpec`] with the specified name, inputs, outputs, and function.
    fn new(
        name: String,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        apply: MethodFunction<Self::Arg>,
    ) -> Self;
    /// Applies the provided arguments to the inner function of the method.
    fn apply(&self, args: Vec<Self::Arg>) -> MethodResult<Self::Arg>;
    /// Returns a reference to the name of the method.
    fn name(&self) -> &str;
}

/// A trait for objects that can act as
/// constraints in a constraint system.
pub trait ConstraintSpec {
    /// The type of the methods of the constraint.
    type Method: MethodSpec;
    /// Constructs a new constraint with the provided methods.
    fn new(methods: Vec<Self::Method>) -> Self;
    /// Returns a reference to the name of the constraint.
    fn name(&self) -> &str;
    /// Returns a slice to the methods of the constraint.
    fn methods(&self) -> &[Self::Method];
    /// Adds a new method to the constraint.
    fn add_method(&mut self, m: Self::Method);
    /// Removes a method from the constraint.
    fn remove_method(&mut self, name: &str);
    /// Returns a slice to the variables used by the methods of the constraint.
    fn variables(&self) -> &[usize];
}

/// Errors that can occur during planning.
#[derive(Debug, PartialEq, Eq)]
pub enum PlanError {
    /// The system was overconstrained, and no plan was found.
    Overconstrained,
}

/// A trait for objects which have the properties of
/// a component, a self-contained subgraph of a constraint system.
/// The most important part is that it contains variables and constraints between them.
pub trait ComponentSpec: Index<&'static str> + IndexMut<&'static str> {
    /// The value of a variable.
    type Value;
    /// The variable type. It has more information than just the [`Value`](Self::Value).
    type Variable;
    /// The type of the constraints of the component.
    type Constraint: ConstraintSpec;
    /// Constructs a new [`ComponentSpec`] with the specified name, values and constraints.
    fn new(
        name: String,
        values: Vec<impl Into<Self::Variable>>,
        constraints: Vec<Self::Constraint>,
    ) -> Self;
    /// Returns the number of variables in the component.
    fn n_variables(&self) -> usize;
    /// Returns the variables of the component.
    fn variables(&self) -> &[Self::Variable];
    /// Returns a specific variable from a component.
    fn get(&self, i: usize) -> &Self::Variable;
    /// Sets the value of a variable.
    fn set(&mut self, i: usize, value: impl Into<Self::Value>);
    /// Returns a slice of the constraints in the component.
    fn constraints(&self) -> &[Self::Constraint];
    /// Returns a mutable slice of the constraints in the component.
    fn constraints_mut(&mut self) -> &mut [Self::Constraint];
    /// Adds a new constraint to the component.
    fn push(&mut self, constraint: Self::Constraint);
    /// Removes the last constraint from the component.
    fn pop(&mut self) -> Option<Self::Constraint>;
    /// Removes a specific constraint from a component.
    fn remove_constraint(&mut self, idx: usize) -> Self::Constraint;
    /// Tries to enforce all constraints in the component.
    fn update(&mut self) -> Result<(), PlanError>
    where
        Self::Value: Send + 'static + Debug;
    /// Converts a variable name to its index in the component.
    fn name_to_idx(&self, name: &str) -> Option<usize>;
}
