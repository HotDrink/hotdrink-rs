//! A module to simplify method construction.

use crate::{MethodFailure, MethodResult};
use itertools::Itertools;
use std::sync::Arc;
use std::{convert::TryInto, fmt::Debug};

/// A parameter for a method.
/// This can either be an immutable reference, or a mutable one.
/// Using this type allows for specifying the type a method expects.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MethodInput {
    /// The method expects an immutable reference.
    Ref(String),
    /// The method expects a mutable reference.
    MutRef(String),
}

impl MethodInput {
    /// Make a parameter representing an immutable reference.
    pub fn make_ref<S: Into<String>>(name: S) -> Self {
        MethodInput::Ref(name.into())
    }

    /// Make a parameter representing a mutable reference.
    pub fn make_mut_ref<S: Into<String>>(name: S) -> Self {
        MethodInput::MutRef(name.into())
    }
}

/// An output of a method.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MethodOutput {
    /// The name of the output variable.
    name: String,
}

impl MethodOutput {
    /// Constructs a new `MethodOutput`.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

/// An argument passed in to a method.
/// This should match the corresponding parameter type.
#[derive(Debug, PartialEq, Eq)]
pub enum MethodArg<'a, T> {
    /// An immutable reference.
    Ref(&'a T),
    /// A mutable reference.
    MutRef(&'a mut T),
}

impl<'a, T> From<&'a T> for MethodArg<'a, T> {
    fn from(r: &'a T) -> Self {
        MethodArg::Ref(r)
    }
}

impl<'a, T> From<&'a mut T> for MethodArg<'a, T> {
    fn from(mr: &'a mut T) -> Self {
        MethodArg::MutRef(mr)
    }
}

/// The actual value could not be converted to the desired one.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConversionError {
    /// The mutability was wrong.
    MutabilityMismatch(MutabilityMismatch),
    /// The type could not be converted.
    TypeConversionFailure,
}

/// The mutability of the argument did not match
/// the mutability that the method expected.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MutabilityMismatch {
    /// Expected an immutable reference, but got a mutable one.
    ExpectedImmutableGotMutable,
    /// Expected a mutable reference, but got an immutable one.
    ExpectedMutableGotImmutable,
}

impl From<MutabilityMismatch> for MethodFailure {
    fn from(mm: MutabilityMismatch) -> Self {
        MethodFailure::MutabilityMismatch(mm)
    }
}

impl From<ConversionError> for MethodFailure {
    fn from(ce: ConversionError) -> Self {
        match ce {
            ConversionError::MutabilityMismatch(mm) => MethodFailure::MutabilityMismatch(mm),
            ConversionError::TypeConversionFailure => {
                MethodFailure::TypeConversionFailure("<VARIABLE>", "<TYPE>")
            }
        }
    }
}

impl<'a, T> MethodArg<'a, T> {
    /// Try to convert his `MethorArg<'a, T>` to a `&'a T`.
    pub fn try_into_ref<U>(self) -> Result<&'a U, ConversionError>
    where
        &'a T: TryInto<&'a U>,
    {
        match self {
            MethodArg::Ref(r) => match r.try_into() {
                Ok(r) => Ok(r),
                Err(_) => Err(ConversionError::TypeConversionFailure),
            },
            _ => Err(ConversionError::MutabilityMismatch(
                MutabilityMismatch::ExpectedImmutableGotMutable,
            )),
        }
    }

    /// Try to convert his `MethorArg<'a, T>` to a `&'a mut T`.
    pub fn try_into_mut<U>(self) -> Result<&'a mut U, ConversionError>
    where
        &'a mut T: TryInto<&'a mut U>,
    {
        match self {
            MethodArg::MutRef(r) => match r.try_into() {
                Ok(r) => Ok(r),
                Err(_) => Err(ConversionError::TypeConversionFailure),
            },
            _ => Err(ConversionError::MutabilityMismatch(
                MutabilityMismatch::ExpectedMutableGotImmutable,
            )),
        }
    }
}

type MethodFunctionInner<T> = Arc<dyn for<'a> Fn(Vec<MethodArg<'a, T>>) -> MethodResult<T>>;

/// A builder for making programmatic construction of methods easier.
#[derive(Clone)]
pub struct MethodBuilder<T> {
    name: String,
    inputs: Vec<MethodInput>,
    outputs: Vec<MethodOutput>,
    apply: Option<MethodFunctionInner<T>>,
    pure: bool,
}

impl<T> MethodBuilder<T> {
    /// Constructs a new `MethodBuilder`.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            apply: None,
            pure: true,
        }
    }

    /// Add an immutable input to the method.
    pub fn input<S: Into<String>>(mut self, input: S) -> Self {
        self.inputs.push(MethodInput::Ref(input.into()));
        self
    }

    /// Add a mutable input to the method.
    pub fn input_mut<S: Into<String>>(mut self, input: S) -> Self {
        self.inputs.push(MethodInput::MutRef(input.into()));
        self
    }

    /// Set the inputs of the method.
    pub fn inputs(mut self, inputs: Vec<MethodInput>) -> Self {
        self.inputs = inputs;
        self
    }

    /// Set the outputs of the method.
    pub fn outputs(mut self, outputs: Vec<MethodOutput>) -> Self {
        self.outputs = outputs.into_iter().map_into().collect();
        self
    }

    /// Set the function to run when this method is applied.
    /// This function takes a slice with a length corresponding to its inputs as input,
    /// and should return a vector of length corresponding to its outputs.
    pub fn apply(
        mut self,
        apply: impl for<'a> Fn(Vec<MethodArg<'a, T>>) -> MethodResult<T> + 'static,
    ) -> Self {
        self.apply = Some(Arc::new(apply));
        self
    }

    /// Set whether this method is pure (referentially transparent) or not.
    ///
    /// This can affect optimization of the method.
    /// If it is pure and none of its inputs have changed,
    /// it will not be re-run during the next update-call.
    /// If it is not pure, it will be re-run every update.
    /// Set this to false if the method reads from or writes to something other than
    /// its inputs and outputs.
    pub fn pure(mut self, pure: bool) -> Self {
        self.pure = pure;
        self
    }
}

impl<T> Debug for MethodBuilder<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Method {}({:?}) -> [{:?}]",
            self.name, self.inputs, self.outputs
        )
    }
}

impl<T> PartialEq for MethodBuilder<T> {
    fn eq(&self, other: &Self) -> bool {
        (&self.name, &self.inputs, &self.outputs) == (&other.name, &other.inputs, &other.outputs)
    }
}

impl<T> Eq for MethodBuilder<T> {}

#[cfg(test)]
mod tests {
    use super::{MethodArg, MethodBuilder, MethodFunctionInner, MethodOutput, MutabilityMismatch};
    use crate::MethodFailure;
    use std::sync::Arc;

    // Define a wrapper struct
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct A;
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct B;

    crate::sum_type! {
        #[derive(Debug, PartialEq)]
        enum AB {
            A,
            B
        }
    }

    #[test]
    fn builder_builds() {
        let mb = MethodBuilder::new("m")
            .input("a")
            .input_mut("b")
            .outputs(vec![MethodOutput::new("c")])
            .apply(|mut v: Vec<MethodArg<'_, _>>| {
                let a: &_ = v.remove(0).try_into_ref()?;
                assert_eq!(a, &3);
                let b: &mut _ = v.remove(0).try_into_mut()?;
                assert_eq!(b, &4);
                Ok(vec![*a + *b])
            });
        if let Some(f) = &mb.apply {
            let result = f(vec![MethodArg::from(&3), MethodArg::from(&mut 4)]);
            assert_eq!(result, Ok(vec![7]))
        }
    }

    #[test]
    fn wrong_mutability_gives_error() {
        // Tries to get mutable reference when it is immutable
        let f: MethodFunctionInner<i32> = Arc::new(|mut v| {
            let a: &mut i32 = v.remove(0).try_into_mut()?;
            Ok(vec![*a])
        });
        let result = f(vec![MethodArg::from(&0)]);
        assert_eq!(
            result,
            Err(MethodFailure::MutabilityMismatch(
                MutabilityMismatch::ExpectedMutableGotImmutable
            ))
        );

        // Tries to get immutable reference when it is mutable
        let f: MethodFunctionInner<i32> = Arc::new(|mut v| {
            let a: &i32 = v.remove(0).try_into_ref()?;
            Ok(vec![*a])
        });
        let result = f(vec![MethodArg::from(&mut 0)]);
        assert_eq!(
            result,
            Err(MethodFailure::MutabilityMismatch(
                MutabilityMismatch::ExpectedImmutableGotMutable
            ))
        );
    }

    #[test]
    fn auto_conversion_in_apply() {
        // Create a function that automatically does conversions.
        let f: MethodFunctionInner<AB> = Arc::new(|mut v| {
            let a: &A = v.remove(0).try_into_ref::<A>()?;
            assert_eq!(a, &A);
            let b: &mut B = v.remove(0).try_into_mut::<B>()?;
            assert_eq!(b, &B);
            Ok(vec![A.into(), B.into()])
        });

        let result = f(vec![
            MethodArg::from(&AB::A(A)),
            MethodArg::from(&mut AB::B(B)),
        ]);

        assert_eq!(result, Ok(vec![AB::A(A), AB::B(B)]));
    }

    #[test]
    fn auto_conversion_may_fail() {
        // Function that tries to get a B
        let f: MethodFunctionInner<AB> = Arc::new(|mut v| {
            let b: &B = v.remove(0).try_into_ref()?;
            Ok(vec![(*b).into()])
        });

        // We pass in an A
        let result = f(vec![MethodArg::from(&AB::A(A))]);

        // Should be type conversion error
        assert_eq!(
            result,
            Err(MethodFailure::TypeConversionFailure("<VARIABLE>", "<TYPE>"))
        );
    }

    #[test]
    fn make_method() {
        let _: MethodBuilder<AB> = crate::method! {
            @impure
            fn m(a: &A, b: &mut B) -> [c, d] {
                let a: &A = a;
                let b: &B = b;
                crate::ret![*a, *b]
            }
        };
    }
}
