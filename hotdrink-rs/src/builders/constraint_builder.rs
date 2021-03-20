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
    ( $method_name:ident ( $( $input:ident $(as $mutability:ident)? : $input_type:ty ),* ) $( -> [ $( $output:ident ),* ] )? $e:block ) => {{
        use $crate::builders::MethodBuilder;
        let fun = $crate::fun!( ( $( $input $( as $mutability )?: $input_type ),* ) { $e });
        MethodBuilder::new(
            stringify!($method_name),
            vec![ $( stringify!($input) ),* ],
            vec![ $( $( stringify!($output) ),* )? ],
            fun,
        )
    }}
}

/// Detects if a type is a mutable reference or not.
/// Note that the type must be a reference.
///
/// # Examples
///
/// ```rust
/// # use crate::is_mut_ref;
/// assert_eq!(is_mut_ref!(&i32), false);
/// assert_eq!(is_mut_ref!(&mut i32), true);
/// ```
#[macro_export]
macro_rules! is_mut_ref {
    (&mut $t:ty) => {
        true
    };
    (&$t:ty) => {
        false
    };
}

/// Introduce the specified variable to scope, with the specified mutability.
#[macro_export]
macro_rules! define_ref {
    ( $name:ident: $type:ty = $value:expr ) => {
        let read_guard = $value.read();
        let $name: $type = &*read_guard;
    };
    ( $name:ident: $type:ty = $value:expr, mut ) => {
        let mut write_guard = $value
            .write()
            .unwrap_or_else(|| panic!("Variable {} was not readable", stringify!($name)));
        let $name: $type = &mut *write_guard;
    };
}

/// A macro for creating a closure that takes a vector as input,
/// while introducing its inputs to the scope automatically.
///
/// # Examples
///
/// ```rust
/// # use hotdrink_rs::fun;
/// assert!(false);
/// ```
#[macro_export]
macro_rules! fun {
    (
        ( $( $input:ident $( as $mutability:ident )? : $input_type:ty ),* ) $e:block ) => {{
        use $crate::builders::value_experiments::Value;
        |values: &[Value<_>]| {
            use $crate::MethodFailure;

            #[allow(unused_mut)]
            let mut counter = 0;
            $(
                // Extract argument
                let value: Option<&Value<_>> = values.get(counter);
                if value.is_none() {
                    return Err(MethodFailure::NoSuchVariable(stringify!($input).to_owned()));
                }
                let value: &Value<_> = value.unwrap();

                // Introduce variable to scope
                if $crate::is_mut_ref!( $input_type ) {
                    let read_guard = value.read();
                    let $input: $input_type = &*read_guard;
                } else {
                    let mut write_guard = value.write().unwrap_or_else(|| panic!("Variable {} was not readable", stringify!($name)));
                    let $input: $input_type = &mut *write_guard;
                }

                #[allow(unused_assignments)]
                counter += 1;
            )*

            $e
        }}
    };
}

#[cfg(test)]
mod tests {
    use super::ConstraintBuilder;
    use crate::{builders::value_experiments::Value, method, MethodFailure};
    use std::sync::{Arc, RwLock};

    #[test]
    fn make_constraint() {
        let _ = ConstraintBuilder::new("Sum")
            .method(method!(m1(a: &i32, b: &i32) -> [c] { Ok(vec![a + b]) }))
            .method(method!(m2(a: &i32, c: &i32) -> [b] { Ok(vec![c - a]) }))
            .method(method!(m3(b: &i32, c: &i32) -> [a] { Ok(vec![c - b]) }));
    }

    #[test]
    fn fun_macro() {
        let add = fun!((a: &i32, b: &i32) { Ok(vec![*a + *b]) });
        let result: Result<Vec<i32>, MethodFailure> =
            add(&[Value::Ref(Arc::new(1)), Value::Ref(Arc::new(2))]);
        assert_eq!(result, Ok(vec![3]));
    }

    #[test]
    fn apply_macro_no_copy() {
        let concat = fun!((a: &String, b: &String) { Ok(vec![a.to_owned() + b.as_str()]) });
        let result: Result<Vec<String>, MethodFailure> = concat(&[
            Value::Ref(Arc::new(String::from("Hello"))),
            Value::Ref(Arc::new(String::from(" World"))),
        ]);
        assert_eq!(result, Ok(vec![String::from("Hello World")]));
    }

    #[test]
    fn define_ref() {
        let value = Value::Ref(Arc::new(3));
        {
            define_ref!(x: &i32 = value);
            assert_eq!(*x, 3);
        }
        {
            define_ref!(x: &mut i32 = value, mut);
            assert_eq!(*x, 3);
        }
    }

    #[test]
    fn modify_mutable_ref() {
        let value = Value::MutRef(Arc::new(RwLock::new(0)));
        let f = fun!((x as mut: &mut i32) {
            *x += 3;
            Ok(vec![])
        });
        let output: Result<Vec<()>, MethodFailure> = f(&[value.clone()]);
        assert_eq!(output, Ok(vec![]));
        assert_eq!(&*value.read(), &3);
    }

    #[test]
    fn detect_mut() {
        assert_eq!(crate::is_mut_ref!(&mut i32), true);
        assert_eq!(crate::is_mut_ref!(&i32), false);

        assert_eq!(crate::is_mut_ref!(&mut String), true);
        assert_eq!(crate::is_mut_ref!(&String), false);

        #[allow(dead_code)]
        struct Foo;

        assert_eq!(crate::is_mut_ref!(&mut Foo), true);
        assert_eq!(crate::is_mut_ref!(&Foo), false);
    }
}
