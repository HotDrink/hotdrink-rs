use std::{collections::HashMap, fmt::Debug};

use crate::data::{
    method::Method,
    traits::{MethodFunction, MethodLike},
};

pub struct RawMethod<'a, T> {
    name: &'a str,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
    apply: MethodFunction<T>,
}

impl<'a, T> RawMethod<'a, T> {
    pub fn new(
        name: &'a str,
        inputs: Vec<&'a str>,
        outputs: Vec<&'a str>,
        apply: MethodFunction<T>,
    ) -> Self {
        Self {
            name,
            inputs,
            outputs,
            apply,
        }
    }

    #[allow(clippy::expect_fun_call)]
    pub fn into_method(self, var_to_idx: &HashMap<String, usize>) -> Method<T>
    where
        T: Clone,
    {
        Method::new(
            self.name.to_string(),
            self.inputs
                .into_iter()
                .map(|i| {
                    var_to_idx
                        .get(i)
                        .expect(&format!("Undefined variable {}", i))
                })
                .copied()
                .collect(),
            self.outputs
                .into_iter()
                .map(|o| {
                    var_to_idx
                        .get(o)
                        .expect(&format!("Undefined variable {}", o))
                })
                .copied()
                .collect(),
            self.apply.clone(),
        )
    }
}

impl<T> PartialEq for RawMethod<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.inputs == other.inputs && self.outputs == other.outputs
    }
}

impl<T> Eq for RawMethod<'_, T> {}

impl<T> Debug for RawMethod<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RawMethod {}({:?} -> {:?})",
            self.name, self.inputs, self.outputs
        )
    }
}
