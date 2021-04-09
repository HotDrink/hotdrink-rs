#![allow(dead_code)]

use bitvec::{bitvec, order::Lsb0, prelude::BitVec};

fn main() {
    let mut component = Component::new(vec![1, 2, 3, 4]);
    component
        .add_constraint({
            let mut constraint = Constraint::new(vec![0, 1, 3]);
            let method = Method::new(&[0, 1], &[2]).unwrap();
            constraint.add_method(method).unwrap();
            constraint
        })
        .unwrap();
    dbg!(component);
}

#[derive(PartialEq, Debug)]
pub enum AddConstraintError {
    UnknownVariables,
    AddMethodError(AddMethodError),
}

impl From<AddMethodError> for AddConstraintError {
    fn from(ame: AddMethodError) -> Self {
        Self::AddMethodError(ame)
    }
}

#[derive(Debug)]
pub struct Component<T> {
    values: Vec<T>,
    constraints: Vec<Constraint>,
}

impl<T> Component<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self {
            values,
            constraints: Vec::new(),
        }
    }
    pub fn add_constraint(&mut self, constraint: Constraint) -> Result<(), AddConstraintError> {
        if constraint.indices.iter().all(|i| *i < self.values.len()) {
            self.constraints.push(constraint);
            Ok(())
        } else {
            Err(AddConstraintError::UnknownVariables)
        }
    }
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
    pub fn n_variables(&self) -> usize {
        self.values.len()
    }
}
#[derive(PartialEq, Debug)]
pub enum AddMethodError {
    MustUseAllVariables,
}

#[derive(Debug)]
pub struct Constraint {
    indices: Vec<usize>,
    references: BitVec<Lsb0, u8>,
    methods: Vec<Method>,
}

pub struct MethodWrapper<'a> {
    constraint: &'a Constraint,
    method: &'a Method,
}

impl<'a> MethodWrapper<'a> {
    pub fn outputs(&self) -> Vec<usize> {
        self.constraint.translate(&self.method.outputs())
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl Constraint {
    pub fn new(indices: Vec<usize>) -> Self {
        let n_variables = indices.len();
        Self {
            indices,
            references: bitvec![Lsb0, u8; 0; n_variables],
            methods: Vec::new(),
        }
    }
    pub fn add_method(&mut self, m: Method) -> Result<(), AddMethodError> {
        if m.inputs_bits().len() != self.indices.len()
            || m.outputs_bits().len() != self.indices.len()
        {
            return Err(AddMethodError::MustUseAllVariables);
        }
        self.references |= m.inputs_bits().to_owned();
        self.references |= m.outputs_bits().to_owned();
        self.methods.push(m);
        Ok(())
    }
    pub fn remove_method(&mut self, index: usize) {
        self.methods.remove(index);
        self.references.clear();
        for m in &self.methods {
            self.references |= m.inputs_bits().to_owned();
            self.references |= m.outputs_bits().to_owned();
        }
    }
    pub fn get_method(&self, index: usize) -> Option<&Method> {
        self.methods.get(index)
    }
    pub fn methods(&self) -> &[Method] {
        &self.methods
    }
    pub fn wrapped_methods(&self) -> Vec<MethodWrapper> {
        self.methods
            .iter()
            .map(|m| MethodWrapper {
                constraint: self,
                method: m,
            })
            .collect()
    }
    pub fn translate(&self, set_indices: &[usize]) -> Vec<usize> {
        set_indices.iter().map(|i| self.indices[*i]).collect()
    }
    pub fn references(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for (i, set) in self.references.iter().enumerate() {
            if *set {
                v.push(self.indices[i]);
            }
        }
        v
    }
}

#[derive(PartialEq, Debug)]
pub enum NewMethodError {
    UnusedVariables,
}

#[derive(PartialEq, Debug)]
pub struct Method {
    inputs: BitVec<Lsb0, u8>,
    outputs: BitVec<Lsb0, u8>,
}

impl Method {
    pub fn new(inputs: &[usize], outputs: &[usize]) -> Result<Self, NewMethodError> {
        // Find size, and ensure all variables are used
        let size = *inputs.iter().chain(outputs.iter()).max().unwrap_or(&0) + 1;
        if !(0..size).all(|i| inputs.contains(&i) || outputs.contains(&i)) {
            return Err(NewMethodError::UnusedVariables);
        }
        let mut self_inputs = bitvec![Lsb0, u8; 0; size];
        for i in inputs {
            self_inputs.set(*i, true);
        }
        let mut self_outputs = bitvec![Lsb0, u8; 0; size];
        for i in outputs {
            self_outputs.set(*i, true);
        }
        let m = Self {
            inputs: self_inputs,
            outputs: self_outputs,
        };
        Ok(m)
    }
    pub fn inputs_bits(&self) -> &BitVec<Lsb0, u8> {
        &self.inputs
    }
    pub fn outputs_bits(&self) -> &BitVec<Lsb0, u8> {
        &self.outputs
    }
    pub fn outputs(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, b) in self.outputs.iter().enumerate() {
            if *b {
                result.push(i);
            }
        }
        result
    }
}

enum MakeComponentError {
    NewMethod(NewMethodError),
    AddMethod(AddMethodError),
    AddConstraint(AddConstraintError),
}

impl From<NewMethodError> for MakeComponentError {
    fn from(e: NewMethodError) -> Self {
        Self::NewMethod(e)
    }
}

impl From<AddMethodError> for MakeComponentError {
    fn from(e: AddMethodError) -> Self {
        Self::AddMethod(e)
    }
}

impl From<AddConstraintError> for MakeComponentError {
    fn from(e: AddConstraintError) -> Self {
        Self::AddConstraint(e)
    }
}

macro_rules! simple_comp {
    (
        component $component_name:ident {
            let $( $var_name:ident = $var_value:expr ),*;
            $(
                constraint $constraint_name:ident {
                $(
                    $method_name:ident ( $( $input:expr ),* => $( $output:expr ),* );
                )*
                }
            )*
        }
    ) => {{
        let make = || -> Result<Component<_>, MakeComponentError> {
            let mut component = Component::new(vec![ $( $var_value ),* ]);
            $(
                let _constraint_name = stringify!($constraint_name);
                let mut constraint = Constraint::new(vec![0, 1, 2]);
                $(
                    let _method_name = stringify!($method_name);
                    let method = Method::new(&[ $( $input ),* ], &[ $( $output ),* ])?;
                    constraint.add_method(method).unwrap();
                )*
                component.add_constraint(constraint)?;
            )*
            Ok(component)
        };
        make()
    }};
}

fn use_macro() {
    let _icomp = simple_comp! {
        component Comp {
            let a = 0, b = 2, c = 5;
            constraint A {
                method(1,2,3 => 4, 5, 6);
            }
        }
    };
    let _scomp = simple_comp! {
        component Comp {
            let a = "", b = "", c = "";
            constraint A {
                method(1,2,3 => 4, 5, 6);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::{AddMethodError, Constraint, Method, NewMethodError};
    use crate::data::traits::{ConstraintLike, MethodLike};
    use pretty_assertions::assert_eq;
    use std::sync::Arc;
    extern crate test;
    use test::Bencher;

    #[test]
    fn new_method_with_ok_io_is_ok() {
        let m = Method::new(&[0, 1], &[2]);
        assert!(m.is_ok());
    }

    #[test]
    fn new_method_with_err_input_is_err() {
        let m = Method::new(&[0, 3], &[2]);
        assert_eq!(m, Err(NewMethodError::UnusedVariables));
    }

    #[test]
    fn new_method_with_err_output_is_err() {
        let m = Method::new(&[0, 1], &[3]);
        assert_eq!(m, Err(NewMethodError::UnusedVariables));
    }

    #[bench]
    fn new_constraint(b: &mut Bencher) -> Result<(), AddMethodError> {
        b.iter(|| {
            const SIZE: usize = 100;
            let mut constraint = Constraint::new((0..SIZE).collect());
            for o in 0..SIZE {
                let mut inputs = Vec::new();
                for i in 0..SIZE {
                    if o != i {
                        inputs.push(i);
                    }
                }
                let method = Method::new(&inputs, &[o]).unwrap();
                constraint.add_method(method).unwrap();
            }
            for o in (0..SIZE).rev() {
                constraint.remove_method(o);
            }
            // constraint.references();
            constraint
        });
        Ok(())
    }

    #[bench]
    fn old_constraint(b: &mut Bencher) -> Result<(), AddMethodError> {
        b.iter(|| {
            const SIZE: usize = 100;
            let mut constraint = crate::data::Constraint::<()>::new(Vec::new());
            for o in 0..SIZE {
                let mut inputs = Vec::new();
                for i in 0..SIZE {
                    if o != i {
                        inputs.push(i);
                    }
                }
                let method = crate::data::Method::new(o.to_string(), inputs, vec![o], Arc::new(Ok));
                constraint.add_method(method);
            }
            for o in (0..SIZE).rev() {
                constraint.remove_method(&o.to_string());
            }
            // constraint.variables();
            constraint
        });
        Ok(())
    }
}
