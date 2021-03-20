//! A builder-struct for programmatically creating components.

use super::{constraint_builder::ConstraintBuilder, value_experiments::Value};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// A builder for making programmatic construction of components easier.
#[derive(Clone, Debug)]
pub struct ComponentBuilder<T> {
    name: String,
    variables: HashMap<String, Value<T>>,
    constraints: Vec<ConstraintBuilder<T>>,
}

impl<T> ComponentBuilder<T> {
    /// Constructs a new `ComponentBuilder` with no variables or constraints.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            variables: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    /// Adds an immutable variable.
    pub fn variable<S: Into<String>>(&mut self, name: S, value: T) -> &mut Self {
        self.variables
            .insert(name.into(), Value::Ref(Arc::new(value)));
        self
    }

    /// Adds a mutable variable.
    pub fn variable_mut<S: Into<String>>(&mut self, name: S, value: T) -> &mut Self {
        self.variables
            .insert(name.into(), Value::MutRef(Arc::new(RwLock::new(value))));
        self
    }

    /// Adds immutable variables.
    pub fn variables<S: Into<String>>(&mut self, variables: Vec<(S, T)>) -> &mut Self {
        variables.into_iter().for_each(|(name, value)| {
            self.variable(name, value);
        });
        self
    }

    /// Adds mutable variables.
    pub fn variables_mut<S: Into<String>>(&mut self, variables: Vec<(S, T)>) -> &mut Self {
        variables.into_iter().for_each(|(name, value)| {
            self.variable_mut(name, value);
        });
        self
    }

    /// Adds a constraint.
    pub fn constraint(&mut self, constraint: ConstraintBuilder<T>) -> &mut Self {
        self.constraints.push(constraint);
        self
    }
}

/// A macro for declaratively making components.
#[macro_export]
macro_rules! build_component {
    (
        component $component_name:ident<$component_type:ty> {
            $( let $( $let_variable:ident : $let_variable_type:ty = $let_value:expr ),*; )?
            $( mut $( $mut_variable:ident : $mut_variable_type:ty = $mut_value:expr ),*; )?
            $(
                constraint $constraint_name:ident {
                    $(
                        $method_name:ident
                            ( $( $input:ident $(as $mutability:ident)? : $input_type:ty ),* )
                            $( -> [ $( $output:ident ),* ] )?
                            $e:block
                    )*
                }
            )*
        }
    ) => {{
        use $crate::builders::{component_builder::ComponentBuilder, constraint_builder::{ConstraintBuilder}};
        let mut component_builder: ComponentBuilder<$component_type> = ComponentBuilder::new(stringify!($component_name));

        // Add immutable variables
        $( $(
            let let_value: $let_variable_type = $let_value.into();
            component_builder.variable(stringify!($let_variable), let_value.into());
        )* )?
        // Add mutable variables
        $( $(
            let mut_value: $mut_variable_type = $mut_value.into();
            component_builder.variable_mut(stringify!($mut_variable), mut_value.into());
        )* )?

        // Add constraints
        $(
            let constraint: ConstraintBuilder<$component_type> = ConstraintBuilder::new(stringify!($constraint_name));
            $(
                // let method = MethodBuilder::new(stringify!($method_name), vec![ $(stringify!($input))* ], vec![ $( $(stringify!($output))* )? ]);
                let method: MethodBuilder<$component_type> = $crate::method!(
                        $method_name
                            ( $( $input$ (as $mutability)? : $input_type ),* )
                            $( -> [ $( $output ),* ] )?
                            $e
                );
                constraint.method(method);
            )*

            component_builder.constraint(constraint);
        )*

        component_builder
    }};
}

#[cfg(test)]
mod tests {
    use super::ComponentBuilder;
    use crate::builders::value_experiments::Value;
    use crate::builders::{ConstraintBuilder, MethodBuilder};
    use crate::method;

    #[test]
    fn builder_builds() {
        let _: &mut ComponentBuilder<i32> = ComponentBuilder::new("Component")
            .variables(vec![("a", 3), ("b", 7)])
            .variable_mut("c", 10)
            .constraint(
                ConstraintBuilder::new("Sum")
                    .method(method!(m1(a: &i32) -> [a] { Ok(vec![*a]) }))
                    .method(method!(m2(a: &i32) -> [a] { Ok(vec![*a]) }))
                    .method(MethodBuilder::new(
                        "m3",
                        vec!["a"],
                        vec!["a"],
                        |v: &[Value<i32>]| {
                            let x = v[0].read();
                            Ok(vec![*x + 2])
                        },
                    )),
            )
            .constraint(ConstraintBuilder::new("Product"));
    }

    #[test]
    fn make_component() {
        // crate::gen_val! {
        //     Foo {
        //         i32,
        //         String,
        //     }
        // }
        // let _ = build_component! {
        //     component Component<Foo> {
        //         let x: i32 = 0, y: i32 = 0, z: i32 = 0;
        //         mut s: String = "abc";
        //         constraint Constraint {
        //             add(x: &i32, y: &i32) -> [] { Ok(vec![]) }
        //             append(s as mut: &mut String) {
        //                 s.push_str("def");
        //                 Ok(vec![])
        //             }
        //         }
        //     }
        // };
    }
}
