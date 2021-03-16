//! Generate a JavaScript wrapper around an inner type to use in a constraint system.

/// Generate a JavaScript wrapper around an inner type to use in a constraint system.
/// [`gen_js_val!`] is to a constraint system made with [`gen_js_constraint_system!`](crate::gen_js_constraint_system!), as
/// [`gen_val!`](hotdrink_rs::gen_val) is to [`ConstraintSystem`](hotdrink_rs::ConstraintSystem).
#[macro_export]
macro_rules! gen_js_val {
    (
        $(#[$outer_meta:meta])*
        $wrapper_vis:vis $wrapper_type:ident {
            $(#[$inner_meta:meta])*
            $inner_vis:vis $inner_type:ident { $( $constr:ident ),* }
        }
    ) => {
        // Generate the inner value
        hotdrink_rs::gen_val! {
            $(#[$inner_meta])*
            $inner_vis $inner_type { $( $constr ),* }
        }

        // Remove the outer sum type and convert the "real" type to `JsValue`.
        impl From<$inner_type> for wasm_bindgen::JsValue {
            fn from(v: $inner_type) -> Self {
                match v {
                    $(
                        $inner_type::$constr(v) => v.into(),
                    )*
                }
            }
        }

        // Generate the JS wrapper
        #[wasm_bindgen::prelude::wasm_bindgen]
        $(#[$outer_meta:meta])*
        #[allow(missing_debug_implementations, non_camel_case_types, missing_docs)]
        $wrapper_vis struct $wrapper_type {
            inner: $inner_type
        }

        // Wrapping and unwrapping
        impl $wrapper_type {
            /// Wraps the inner type.
            pub fn wrap(inner: $inner_type) -> Self {
                Self { inner }
            }
            /// Unwraps the inner type.
            pub fn unwrap(self) -> $inner_type {
                self.inner
            }
        }

        // Generate constructors to use from JS,
        // since we can not use the enum constructors directly.
        #[wasm_bindgen::prelude::wasm_bindgen]
        #[allow(non_snake_case, missing_docs)]
        impl $wrapper_type {
            $(
                #[wasm_bindgen::prelude::wasm_bindgen]
                pub fn $constr(v: $constr) -> Self {
                    $wrapper_type {
                        inner: $inner_type::$constr(v)
                    }
                }
            )*
        }
    };
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn gen_js_val_example_compiles() {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        #[derive(PartialEq, Clone, Debug)]
        pub struct MyCircle {
            x: usize,
            y: usize,
            r: usize,
        }

        gen_js_val! {
            pub MyWrapper {
                #[derive(Debug, PartialEq, Clone)]
                pub MyType { i32, MyCircle }
            }
        };
    }
}
