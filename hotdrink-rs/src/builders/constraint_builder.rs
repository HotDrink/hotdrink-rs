//! A builder-struct for programmatically creating constraints.

use super::method_builder::MethodBuilder;
use std::fmt::Debug;

/// A builder for making programmatic construction of constraints easier.
#[derive(Clone, Debug)]
pub struct ConstraintBuilder<T> {
    name: String,
    methods: Vec<MethodBuilder<T>>,
}

impl<T> ConstraintBuilder<T> {
    /// Constructs a new `ConstraintBuilder`.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            methods: Vec::new(),
        }
    }
    /// Adds a method to the builder.
    pub fn method(mut self, method: MethodBuilder<T>) -> Self {
        self.methods.push(method);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::builders::ConstraintBuilder;

    #[test]
    fn make_constraint() {
        let _: ConstraintBuilder<i32> = ConstraintBuilder::new("Sum")
            .method(method!(
                fn m1(a: &i32, b: &i32) -> [c] {
                    Ok(vec![*a + *b])
                }
            ))
            .method(method!(
                fn m2(a: &i32, c: &i32) -> [b] {
                    Ok(vec![*c - *a])
                }
            ))
            .method(method!(
                fn m3(b: &i32, c: &mut i32) -> [a] {
                    Ok(vec![*c - *b])
                }
            ));
    }
}
