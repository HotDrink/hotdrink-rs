//! A macro for specifying components.

/// A macro for specifying components.
///
/// This can be used to construct constraint systems declaratively by
/// combining the desired components in a [`ConstraintSystem`](crate::model::ConstraintSystem).
///
/// # Examples
///
/// ```rust
/// # use hotdrink_rs::{model::Component, component, ret};
/// let component: Component<i32> = component! {
///     component SumAndProduct {
///         let a: i32 = 0, b: i32 = 0, c: i32 = 0, d: i32 = 0;
///         constraint Sum {
///             sum1(a: &i32, b: &i32) -> [c] = ret![*a + *b];
///             sum2(a: &i32, c: &i32) -> [b] = ret![*c - *a];
///             sum3(b: &i32, c: &i32) -> [a] = ret![*c - *b];
///         }
///         constraint Product {
///             product1(a: &i32, b: &i32) -> [d] = ret![*a * *b];
///             product2(a: &i32, d: &i32) -> [b] = ret![*d / *a];
///             product3(b: &i32, d: &i32) -> [a] = ret![*d / *b];
///         }
///     }
/// };
/// ```
#[macro_export]
macro_rules! component {
    (@value_or_default: $t:ty ) => {{ <$t>::default() }};
    (@value_or_default: $t:ty = $value:expr) => {{ $value }};
    (
        // Match a component, its name, and constraints.
        component $component_name:ident {
            // Match variables, their types, and default values.
            let $($i:ident: $val_ty:ty $( = $e:expr )? ),*;
            $(
                // Match a constraint, its name, and methods.
                constraint $constraint_name:ident {
                    // Match a precondition of the constraint.
                    $( precondition $precondition:expr; )?
                    // Match a postcondition of the constraint.
                    $( postcondition $postcondition:expr; )?
                    $(
                        // Match a method, its inputs, outputs and body.
                        $method_name:ident
                            ($($inp:ident: $inp_ty:ty),*)
                            $(-> [$($out:ident),+])?
                            = $m_expr:expr;
                    )+
                }
            )*
        }
    ) => {{
        let variables = vec![ $( stringify!($i) ),* ];
        let values = vec![ $( {
            let value: $val_ty = $crate::component!(@value_or_default: $val_ty $( = $e.into() )?);
            value.into()
        }
        ),* ];
        // Component
        $crate::macros::RawComponent::new(
            stringify!($component_name),
            variables,
            values,
            // Constraints
            vec![ $(
                $crate::macros::RawConstraint::new_with_assert(
                    stringify!($constraint_name),
                    // Methods
                    vec![ $(
                        #[allow(unused_mut)]
                        $crate::macros::RawMethod::new(
                            stringify!($method_name),
                            vec![ $( stringify!($inp) ),* ],
                            vec![ $( $( stringify!($out) ),* )? ],
                            {
                                #[allow(unused_assignments)]
                                std::sync::Arc::new(move |values| {
                                    let mut var_idx = 0;
                                    $(
                                        // Assign the value to the variable
                                        let $inp = &values.get(var_idx);
                                        // Verify that it exists
                                        if $inp.is_none() {
                                            return Err($crate::planner::MethodFailure::NoSuchVariable(stringify!($inp).to_owned()));
                                        }
                                        // Convert it to the appropriate type
                                        let $inp = std::convert::TryInto::<$inp_ty>::try_into(&**$inp.unwrap());
                                        // Verify that it worked
                                        if $inp.is_err() {
                                            return Err($crate::planner::MethodFailure::TypeConversionFailure(stringify!($inp), stringify!($inp_ty)));
                                        }
                                        let $inp = $inp.unwrap();

                                        var_idx += 1;
                                    )*

                                    use std::sync::Arc;
                                    let e: $crate::planner::MethodResult<_> = $m_expr;
                                    // Evaluate user code and wrap in [`Arc`]s
                                    e.map(|v| v.into_iter().map(Arc::new).collect())
                                })
                            }
                        )
                    ),* ], // End of methods
                    None,
                )
            ),* ] // End of constraints
        ).into_component()
    }};
}

/// Turns a list of inputs into a successful [`MethodResult`]().
/// This can be used defining methods in components with [`component!`].
/// To make returning the possible values of a sum type used in a [`Component`](crate::model::Component) easier,
/// it will automatically call [`Into::into`] on each argument.
///
/// # Examples
///
/// [`ret!`] can be used with normal values like [`i32`].
///
/// ```rust
/// # use std::sync::Arc;
/// # use hotdrink_rs::{ret, planner::MethodResult};
/// let result: MethodResult<i32> = ret![3, 5];
/// assert_eq!(result, Ok(vec![3, 5]));
/// ```
///
/// It can also be used with enums.
///
/// ```rust
/// # use std::sync::Arc;
/// # use hotdrink_rs::{ret, planner::MethodResult};
/// # #[derive(Debug, PartialEq)]
/// enum Shape {
///     Circle(usize),
///     Square(usize, usize),
/// }
/// let result: MethodResult<Shape> = ret![Shape::Circle(3), Shape::Square(4, 5)];
/// assert_eq!(result, Ok(vec![Shape::Circle(3), Shape::Square(4, 5)]));
/// ```
///
/// Even with wrapper types that implement [`From::from`] its variants.
/// These values can then be used directly in [`ret!`],
/// and they will automatically be converted if possible.
///
/// ```rust
/// # use std::sync::Arc;
/// # use hotdrink_rs::{ret, planner::MethodResult};
/// # #[allow(non_camel_case_types)]
/// # #[derive(Debug, PartialEq)]
/// enum Value {
///     i32(i32),
///     f64(f64),
/// }
///
/// // impl From<i32> for Value { ... }
/// # impl From<i32> for Value {
/// #     fn from(n: i32) -> Self {
/// #         Value::i32(n)
/// #     }
/// # }
///
/// // impl From<f64> for Value { ... }
/// # impl From<f64> for Value {
/// #     fn from(n: f64) -> Self {
/// #         Value::f64(n)
/// #     }
/// # }
///
/// let result: MethodResult<Value> = ret![3i32, 5.0f64];
/// assert_eq!(result, Ok(vec![Value::i32(3), Value::f64(5.0)]));
/// ```
#[macro_export]
macro_rules! ret {
    ($($e:expr),*) => {{ Ok(vec![$($e.into()),*]) }}
}

/// Turns a list of inputs into a failed [`MethodResult`]().
/// This can be used defining methods in components with [`component!`].
///
/// # Examples
///
/// ```rust
/// # use hotdrink_rs::{fail, planner::{MethodResult, MethodFailure}};
/// let result: MethodResult<()> = fail!("Expected {} to be equal to {}", 2, 3);
/// assert_eq!(result, Err(MethodFailure::Custom(String::from("Expected 2 to be equal to 3"))));
/// ```
#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {{
        // let err: $crate::planner::MethodResult<_> = Err($crate::planner::MethodFailure::Custom(format!($($arg)*)));
        // err
        Err($crate::planner::MethodFailure::Custom(format!($($arg)*)))
    }};
}

#[cfg(test)]
mod tests {
    use crate::{
        component_type,
        model::Component,
        planner::{ComponentSpec, ConstraintSpec, MethodFailure, MethodSpec},
    };
    use std::convert::TryFrom;

    macro_rules! all_into {
        ($($e:expr),*) => {{ vec![$(std::sync::Arc::new($e.into())),*] }}
    }

    // Generate an enum for standard types
    component_type! {
        #[derive(Debug, PartialEq, Clone)]
        enum Standard { i32, f64, String }
    }

    // A custom type
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Circle {
        x: usize,
        y: usize,
        r: usize,
    }

    // Generate an enum for custom types
    component_type! {
        #[derive(Debug, PartialEq, Clone)]
        enum Custom { i32, Circle }
    }

    #[test]
    fn gen_val_allows_standard_types() {
        let i = 4;
        assert_eq!(Standard::from(i), Standard::i32(i));
        let f = 13.0;
        assert_eq!(Standard::from(f), Standard::f64(f));
    }

    #[test]
    fn gen_val_allows_custom_types() {
        let c = Circle { x: 3, y: 8, r: 10 };
        let v = Custom::from(c);
        assert_eq!(v, Custom::Circle(c));
        assert_eq!(TryFrom::try_from(&v), Ok(&c));
    }

    #[test]
    fn methods_automatically_unwrap_arguments() {
        let comp: Component<Standard> = component! {
            component comp {
                let i: i32 = 4, s: String = "abc";
                constraint constr {
                    m(i: &i32, s: &String) = {
                        assert_eq!(i, &4);
                        assert_eq!(s, &"abc".to_string());
                        ret![]
                    };
                }
            }
        };

        let constr = comp.constraints()[0].clone();
        let m = constr.methods()[0].clone();
        assert_eq!(m.apply(all_into![4, "abc".to_string()]), Ok(vec![]));
    }

    #[ignore = "This can not be verified with `apply` anymore. Must be done higher up."]
    #[test]
    fn methods_fail_when_undefined_variable() {
        component_type! {
            enum Value { i32, String }
        }

        #[allow(unused_variables)]
        let comp: Component<Standard> = component! {
            component comp {
                let i: i32 = 4, s: String = "abc";
                constraint constr {
                    m(not_defined: &i32) = ret![];
                }
            }
        };

        let constr = comp.constraints()[0].clone();
        let m = constr.methods()[0].clone();
        assert_eq!(
            m.apply(all_into![4]),
            Err(MethodFailure::NoSuchVariable("not_defined".to_string()))
        );
    }

    #[ignore = "This can not be verified with `apply` anymore. Must be done higher up."]
    #[test]
    fn methods_fail_when_wrong_type() {
        #[allow(unused_variables)]
        let comp: Component<Standard> = component! {
            component comp {
                let i: i32 = 4, s: String = "abc";
                constraint constr {
                    m(s: &i32) = ret![];
                }
            }
        };

        let constr = comp.constraints()[0].clone();
        let m = constr.methods()[0].clone();
        assert_eq!(
            m.apply(all_into![0]),
            Err(MethodFailure::TypeConversionFailure("s", "&i32"))
        );
    }

    #[test]
    fn method_output_is_saved() {
        let comp: Component<Standard> = component! {
            component comp {
                let i: i32 = 4, s: String = "abc";
                constraint constr {
                    m(i: &i32) -> [i] = {
                        ret![2*i]
                    };
                }
            }
        };

        let constr = &comp.constraints()[0].clone();
        let m = &constr.methods()[0].clone();
        assert_eq!(m.apply(all_into![3]), Ok(all_into![6]));
    }

    #[test]
    fn component_macro_provides_default_values() {
        let _: Component<i32> = component! {
            component Comp {
                let i: i32 = 0, j: i32;
            }
        };
        let _: Component<String> = component! {
            component Comp {
                let i: String = "abc", j: String;
            }
        };
    }
}
