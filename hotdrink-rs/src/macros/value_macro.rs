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
            impl std::convert::From<&$constr> for $type_name {
                fn from(x: &$constr) -> Self {
                    $type_name::$constr(x.to_owned())
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
