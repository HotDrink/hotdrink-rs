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

/// A macro for creating a `MethodBuilder`.
#[macro_export]
macro_rules! method {
    ( @impure $( $rest:tt )*) => { $crate::method!($($rest)*).pure(false) };
    (
        $method_name:ident ( $( $params:tt )* ) $( -> [ $( $output:ident ),* ] )? $e:block
    ) => {{
        use $crate::builders::{MethodBuilder, method_builder::{MethodParam}};
        MethodBuilder::new(stringify!($method_name))
            .inputs( $crate::make_params!( $( $params )* ) )
            .outputs( vec![ $( $( stringify!($output) ),* )? ] )
            .apply(|mut values| {
                $crate::define_refs!(values, $($params)*);
                $e
            })
    }};
}

/// Turn a parameter list into a list of [`MethodParam`](crate::builders::method_builder::MethodParam).
#[macro_export]
macro_rules! make_params {
    () => {{ Vec::new() }};
    ( $name:ident: & $t:ty $(, $($rest:tt)* )? ) => {{
        let mut v = vec![MethodParam::make_ref(stringify!($name))];
        v.extend($crate::make_params!( $( $( $rest )* )? ));
        v
    }};
    ( $name:ident: &mut $t:ty $(, $($rest:tt)* )? ) => {{
        let mut v = vec![MethodParam::make_mut_ref(stringify!($name))];
        v.extend($crate::make_params!( $( $( $rest )* )? ));
        v
    }};
}

/// Introduce references to the specified parameters.
#[macro_export]
macro_rules! define_refs {
    ( $values:expr ) => {{}};
    ( $values:expr, $name:ident: & $t:ty $(, $($rest:tt)* )? ) => {
        // Get reference and try to convert it
        let $name: &$t = $values.remove(0).try_into_ref()?;
        $crate::define_refs!($values $(, $($rest)*)?);
    };
    ( $values:expr, $name:ident: &mut $t:ty $(, $($rest:tt)* )? ) => {
        // Get reference and try to convert it
        let $name: &mut $t = $values.remove(0).try_into_mut()?;
        $crate::define_refs!($values $(, $($rest)*)?);
    };
}

#[cfg(test)]
mod tests {
    use crate::builders::{method_builder::MethodParam, ConstraintBuilder};

    #[test]
    fn make_constraint() {
        let _: ConstraintBuilder<i32> = ConstraintBuilder::new("Sum")
            .method(crate::method!(m1(a: &i32, b: &i32) -> [c] { Ok(vec![*a + *b]) } ))
            .method(crate::method!(m2(a: &i32, c: &i32) -> [b] { Ok(vec![*c - *a]) } ))
            .method(crate::method!(m3(b: &i32, c: &mut i32) -> [a] { Ok(vec![*c - *b]) } ));
    }

    #[test]
    fn make_params() {
        let _: Vec<MethodParam> = make_params!();
        assert_eq!(make_params!(a: &i32), vec![MethodParam::make_ref("a")]);
        assert_eq!(
            make_params!(a: &mut i32),
            vec![MethodParam::make_mut_ref("a")]
        );
        assert_eq!(
            make_params!(a: &i32, b: &mut i32),
            vec![MethodParam::make_ref("a"), MethodParam::make_mut_ref("b")]
        );

        let many = make_params!(
            a: &i32,
            b: &mut i32,
            c: &i32,
            d: &mut String,
            e: &std::collections::HashMap,
            f: &mut std::collections::HashMap
        );
        assert_eq!(
            many,
            vec![
                MethodParam::make_ref("a"),
                MethodParam::make_mut_ref("b"),
                MethodParam::make_ref("c"),
                MethodParam::make_mut_ref("d"),
                MethodParam::make_ref("e"),
                MethodParam::make_mut_ref("f")
            ]
        );
    }
}
