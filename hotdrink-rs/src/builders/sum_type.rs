//! A macro for easily generating conversion traits on a sum type.

use crate::planner::MethodFailure;

/// The enum was not the desired variant, and could not be converted.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct WrongEnumVariant(pub &'static str, pub &'static str);

impl From<WrongEnumVariant> for MethodFailure {
    fn from(wev: WrongEnumVariant) -> Self {
        let WrongEnumVariant(from, to) = wev;
        MethodFailure::TypeConversionFailure(from, to)
    }
}

/// Construct a sum type (enum) with the specified variants.
/// This macro will automatically generate many conversion traits:
/// For an enum Foo, and all variants Variant, it will generate
/// 1. From<Variant> to Foo.
/// 2. TryFrom<&'a Foo> to &'a Variant.
/// 3. TryFrom<&'a mut Foo> to &'a mut Variant.
macro_rules! sum_type {
    (
        $( #[$meta:meta] )*
        $visibility:vis enum $name:ident {
            $( $variant:ident ),* $(,)?
        }
    ) => {
        $( #[$meta] )*
        #[allow(non_camel_case_types)]
        $visibility enum $name {
            $( $variant($variant) ),*
        }

        $(
            // Generate From
            impl <'a> std::convert::From<$variant> for $name {
                fn from(variant: $variant) -> Self {
                    $name::$variant(variant)
                }
            }

            // Generate From ref
            impl<'a> std::convert::TryFrom<&'a $name> for &'a $variant {
                type Error = $crate::builders::sum_type::WrongEnumVariant;
                fn try_from(name: &'a $name) -> Result<Self, Self::Error> {
                    match name {
                        $name::$variant(value) => Ok(value),
                        _ => Err($crate::builders::sum_type::WrongEnumVariant(stringify!($name), stringify!($variant))),
                    }
                }
            }

            // Generate from mut ref
            impl<'a> std::convert::TryFrom<&'a mut $name> for &'a mut $variant {
                type Error = $crate::builders::sum_type::WrongEnumVariant;
                fn try_from(name: &'a mut $name) -> Result<Self, Self::Error> {
                    match name {
                        $name::$variant(value) => Ok(value),
                        _ => Err($crate::builders::sum_type::WrongEnumVariant(stringify!($name), stringify!($variant))),
                    }
                }
            }
        )*
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    #[test]
    fn use_macro() {
        #[derive(Debug, PartialEq)]
        struct A;
        #[derive(Debug, PartialEq)]
        struct B;
        sum_type! {
            #[derive(Debug, PartialEq)]
            enum AB {
                A,
                B
            }
        }

        let ab = &AB::A(A);
        let a: Result<&A, _> = ab.try_into();
        assert_eq!(a, Ok(&A));

        let ab = &mut AB::B(B);
        let b: Result<&mut B, _> = ab.try_into();
        assert_eq!(b, Ok(&mut B));
    }
}
