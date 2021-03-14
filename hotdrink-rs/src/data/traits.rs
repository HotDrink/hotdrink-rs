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

pub trait MethodLike: Vertex {
    type Arg;
    fn new(
        name: String,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        apply: MethodFunction<Self::Arg>,
    ) -> Self;
    fn apply(&self, args: Vec<Self::Arg>) -> MethodResult<Self::Arg>;
    fn name(&self) -> &str;
}

pub trait ConstraintLike {
    type Method: MethodLike;
    fn new(methods: Vec<Self::Method>) -> Self;
    fn name(&self) -> &str;
    fn methods(&self) -> &[Self::Method];
    fn add_method(&mut self, m: Self::Method);
    fn remove_method(&mut self, name: &str);
    fn variables(&self) -> &[usize];
}

/// Errors that can occur during solving.
#[derive(Debug, PartialEq, Eq)]
pub enum PlanError {
    /// The system was overconstrained, and no plan was found.
    Overconstrained,
}

pub trait ComponentLike: Index<&'static str> + IndexMut<&'static str> {
    type Value;
    type Variable;
    type Constraint: ConstraintLike;
    fn new(
        name: String,
        values: Vec<impl Into<Self::Variable>>,
        constraints: Vec<Self::Constraint>,
    ) -> Self;
    fn n_variables(&self) -> usize;
    fn values(&self) -> &[Self::Variable];
    fn get(&self, i: usize) -> &Self::Variable;
    fn set(&mut self, i: usize, value: impl Into<Self::Value>);
    fn constraints(&self) -> &[Self::Constraint];
    fn constraints_mut(&mut self) -> &mut [Self::Constraint];
    fn push(&mut self, constraint: Self::Constraint);
    fn pop(&mut self) -> Option<Self::Constraint>;
    fn ranking(&self) -> Vec<usize>;
    fn update(&mut self) -> Result<(), PlanError>
    where
        Self::Value: Send + 'static + Debug;
    fn name_to_idx(&self, name: &str) -> Option<usize>;
    fn remove_constraint(&mut self, idx: usize) -> Self::Constraint;
}
