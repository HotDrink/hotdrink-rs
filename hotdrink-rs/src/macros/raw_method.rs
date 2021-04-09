//! Types and operations for creating [`RawMethod`]s that are easier to make manually than [`Method`].
//! They can then be converted to real [`Method`]s later.
//!
//! [`Method`]: crate::model::Method

use crate::model::Method;
use crate::planner::{MethodFunction, MethodSpec};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Debug};

/// An intermediate struct for constructing [`Method`]s.
pub struct RawMethod<T> {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    apply: MethodFunction<T>,
}

impl<T> RawMethod<T> {
    /// Constructs a new [`RawMethod`].
    pub fn new<S: Into<String>>(
        name: S,
        inputs: Vec<S>,
        outputs: Vec<S>,
        apply: MethodFunction<T>,
    ) -> Self {
        Self {
            name: name.into(),
            inputs: inputs.into_iter().map_into().collect(),
            outputs: outputs.into_iter().map_into().collect(),
            apply,
        }
    }

    /// Converts this [`RawMethod`] into a [`Method`].
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
                        .get(&i)
                        .expect(&format!("Undefined variable {}", i))
                })
                .copied()
                .collect(),
            self.outputs
                .into_iter()
                .map(|o| {
                    var_to_idx
                        .get(&o)
                        .expect(&format!("Undefined variable {}", o))
                })
                .copied()
                .collect(),
            self.apply.clone(),
        )
    }
}

impl<T> PartialEq for RawMethod<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.inputs == other.inputs && self.outputs == other.outputs
    }
}

impl<T> Eq for RawMethod<T> {}

impl<T> Debug for RawMethod<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RawMethod {}({:?} -> {:?})",
            self.name, self.inputs, self.outputs
        )
    }
}
