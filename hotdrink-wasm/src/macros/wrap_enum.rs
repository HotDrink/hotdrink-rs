//! Generate a struct wrapper around an enum, since enums can not be exposed to WebAssembly.

/// Generate a struct wrapper around an enum, since enums can not be exposed to WebAssembly.
/// You will likely not have to use this macro directly, as it is called by the constraint system wrappers.
///
/// # Examples
///
/// In this example, we generate a sum type `MyEnum` with two variants.
/// We can not expose the enum directly to WebAssembly, and thus need a wrapper.
/// The wrapper will be generated with two functions: `MyWrapper::i32` and `MyWrapper::String`
/// that will call the constructors `MyEnum::i32` and `MyEnum::String` internally.
///
/// ```rust
/// hotdrink_wasm::wrap_enum! {
///     pub struct MyWrapper {
///         pub enum MyEnum {
///             i32,
///             String
///         }
///     }
/// }
/// let _: MyWrapper = MyWrapper::i32(5);
/// let _: MyWrapper = MyWrapper::String("Hello".to_string());
/// ```
#[macro_export]
macro_rules! wrap_enum {
    (
        $(#[$outer_meta:meta])*
        $wrapper_vis:vis struct $wrapper_type:ident {
            $(#[$inner_meta:meta])*
            $inner_vis:vis enum $inner_type:ident { $( $constr:ident ),* }
        }
    ) => {
        // Generate the inner value
        hotdrink_rs::component_type! {
            $(#[$inner_meta])*
            $inner_vis enum $inner_type { $( $constr ),* }
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
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub struct MyCircle {
            x: usize,
            y: usize,
            r: usize,
        }

        wrap_enum! {
            pub struct MyWrapper {
                #[derive(Debug, PartialEq, Clone)]
                pub enum MyType { i32, MyCircle }
            }
        };
    }
}
