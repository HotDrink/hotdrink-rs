//! A macro for automatically generating an enum that has a variation for each type to use in a constraint system.

/// A macro for automatically generating an enum that has a variation for each type to use in a constraint system.
/// It will also implement [`From`] and [`TryInto`](std::convert::TryInto) implementations for each of these variations.
///
/// # Examples
/// ```rust
/// # use std::convert::TryInto;
/// #[derive(Debug, PartialEq)]
/// struct Foo;
///
/// // Generate the struct and impls
/// hotdrink_rs::gen_val! {
///     #[derive(Debug, PartialEq)]
///     MyType {
///       i32,
///       f64,
///       Foo
///     }
/// }
///
/// // Create instance of MyType
/// let mt: MyType = MyType::from(Foo);
/// assert_eq!(mt, MyType::Foo(Foo));
///
/// // Try to convert it to f32
/// let x: Result<f64, ()> = mt.try_into();
/// assert_eq!(x, Err(()));
///
/// // Try to convert a MyType to i32
/// let y: Result<i32, ()> = MyType::from(23).try_into();
/// assert_eq!(y, Ok(23));
/// ```
#[macro_export]
macro_rules! gen_val {
    (
        $(#[$meta:meta])*
        $vis:vis $type_name:ident { $( $constr:ident ),* }
    ) => {
        // Generate enum
        $(#[$meta])*
        #[allow(non_camel_case_types)]
        $vis enum $type_name {
            $( $constr($constr) ),*
        }

        // Generate From impls
        $(
            impl std::convert::From<$constr> for $type_name {
                fn from(x: $constr) -> Self {
                    $type_name::$constr(x)
                }
            }
        )*

        // Generate TryFrom impls
        $(
            // From reference
            impl<'a> std::convert::TryFrom<&'a $type_name> for &'a $constr {
                type Error = ();
                fn try_from(value: &'a $type_name) -> Result<Self, Self::Error> {
                    match value {
                        $type_name::$constr(x) => Ok(x),
                        #[allow(unreachable_patterns)]
                        _ => Err(()),
                    }
                }
            }
            // From value
            impl std::convert::TryFrom<$type_name> for $constr {
                type Error = ();
                fn try_from(value: $type_name) -> Result<Self, Self::Error> {
                    match value {
                        $type_name::$constr(x) => Ok(x),
                        #[allow(unreachable_patterns)]
                        _ => Err(()),
                    }
                }
            }
        )*
    };
}
