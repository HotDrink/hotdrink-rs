//! A signup example.

use hotdrink_rs::{
    component, fail,
    model::{Component, ConstraintSystem},
    ret,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// Generate a type for values in the constraint system.
// The constraint system can only hold values of a single type,
// so we must create an enum [`Number`] that has the variants we need.
// We can not expose non-C-style enums with `wasm-bindgen`, so we must use [`NumberWrapper`]
// to construct the variants instead, using the generated functions `NumberWrapper::i32`
// and `NumberWrapper::f64` from JavaScript.
hotdrink_wasm::component_type_wrapper! {
    pub struct SignupValueWrapper {
        #[derive(Debug, Clone)]
        pub enum SignupValue {
            String,
            bool
        }
    }
}

// Generate a wrapper around the constraint system.
// We must specify the generic argument of the constraint system at compile time
// to be able to expose the type with `wasm-bindgen`.
hotdrink_wasm::constraint_system_wrapper!(SignupCs, SignupValueWrapper, SignupValue);

/// Generate a component that describes the constraints
/// required between properties of the image.
pub fn signup_component() -> Component<SignupValue> {
    component! {
        component Signup {
            let username: String = "Henry", email: String = "something@somewhere.com",
                password1: String = "password", password2: String = "password",
                equal_passwords: bool = true,
                agreed: bool,
                button_enabled: bool;

            constraint ValidUsername {
                validate(username: &String) -> [username] = {
                    if username.is_empty() {
                        return fail!("Username can not be empty");
                    }
                    if !username.chars().all(|c| c.is_alphanumeric()) {
                        return fail!("Username must be alphanumeric");
                    }
                    ret![username.clone()]
                };
            }

            constraint ValidEmail {
                validate(email: &String) -> [email] = {
                    if email.contains("@") {
                        ret![email.clone()]
                    } else {
                        fail!("Email must contain @")
                    }
                };
            }

            constraint ValidPassword1 {
                validate(password1: &String) -> [password1] = {
                    if password1.len() < 6 || 8 < password1.len() {
                        return fail!("Length must be between 6 and 8 characters");
                    }
                    ret![password1.clone()]
                };
            }

            constraint ValidPassword2 {
                validate(password2: &String) -> [password2] = {
                    if password2.len() < 6 || 8 < password2.len() {
                        return fail!("Length must be between 6 and 8 characters");
                    }
                    ret![password2.clone()]
                };
            }

            constraint EqualPasswords {
                equal(password1: &String, password2: &String) -> [equal_passwords] = {
                    ret![password1 == password2]
                };
            }

            constraint ButtonEnabled {
                check(username: &String, email: &String, equal_passwords: &bool, agreed: &bool) -> [button_enabled] = {
                    ret![*equal_passwords && *agreed]
                };
            }
        }
    }
}

/// Adds the component to a [`ConstraintSystem`],
/// then wraps that in the [`NumberJsCs`].
#[wasm_bindgen]
pub fn signup() -> Result<SignupCs, JsValue> {
    let mut cs = ConstraintSystem::new();
    cs.add_component(signup_component());
    SignupCs::wrap(cs)
}
