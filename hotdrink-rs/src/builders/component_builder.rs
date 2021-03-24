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
    (@value_or_default: $t:ty ) => {{ <$t>::default() }};
    (@value_or_default: $t:ty = $value:expr) => {{ $value }};
    (
        component $component_name:ident<$sum_type:ty> {
            $( let $( $let_variable:ident : $let_variable_type:ty $( = $let_value:expr )? ),* ; )?
            $( mut $( $mut_variable:ident : $mut_variable_type:ty $( = $mut_value:expr )? ),* ; )?
            $(
                constraint $constraint_name:ident {
                    $(
                        $method_name:ident
                            ( $( $param:tt )* )
                            $( -> [ $( $output:ident ),* ] )?
                            $e:block
                    )*
                }
            )*
        }
    ) => {{
        use $crate::builders::{ComponentBuilder, ConstraintBuilder, MethodBuilder};

        #[allow(unused_mut)]
        let mut component_builder: ComponentBuilder<$sum_type> = ComponentBuilder::new(stringify!($component_name));

        // Add immutable variables
        $( $(
            let let_value: $let_variable_type = ($crate::build_component!(@value_or_default: $let_variable_type $( = $let_value )?)).into();
            component_builder.variable(stringify!($let_variable), let_value.into());
        )* )?
        // Add mutable variables
        $( $(
            let mut_value: $mut_variable_type = ($crate::build_component!(@value_or_default: $mut_variable_type $( = $mut_value )?)).into();
            component_builder.variable_mut(stringify!($mut_variable), mut_value.into());
        )* )?

        // Add constraints
        $(
            #[allow(unused_mut)]
            let mut constraint: ConstraintBuilder<$sum_type> = ConstraintBuilder::new(stringify!($constraint_name));
            $(
                // let method = MethodBuilder::new(stringify!($method_name), vec![ $(stringify!($input))* ], vec![ $( $(stringify!($output))* )? ]);
                let method: MethodBuilder<$sum_type> = $crate::method!(
                        $method_name <$sum_type>
                            ( $( $param )* )
                            $( -> [ $( $output ),* ] )?
                            $e
                );
                constraint = constraint.method(method);
            )*

            component_builder.constraint(constraint);
        )*

        component_builder
    }};
}

#[cfg(test)]
mod tests {
    use super::ComponentBuilder;
    use crate::builders::ConstraintBuilder;
    use crate::method;

    #[test]
    fn builder_builds() {
        let _: &mut ComponentBuilder<i32> = ComponentBuilder::new("Component")
            .variables(vec![("a", 3), ("b", 7)])
            .variable_mut("c", 10)
            .constraint(
                ConstraintBuilder::new("Sum")
                    .method(method!(m1(a: &i32) -> [b] { Ok(vec![*a]) }))
                    .method(method!(m2(b: &mut i32) -> [a] { Ok(vec![*b]) })),
            )
            .constraint(ConstraintBuilder::new("Product"));
    }

    #[test]
    fn make_component() {
        crate::sum_type! {
            #[derive(Debug)]
            enum Foo {
                i32,
                f64,
                String,
            }
        }
        let comp = build_component! {
            component Component<Foo> {
                let x: i32 = 0, y: i32, z: f64;
                mut s: String;
                constraint Constraint {
                    m() -> [] { Ok(vec![]) }
                    add(x: &i32, y: &i32) -> [] { Ok(vec![]) }
                    append(s: &mut String) {
                        s.push_str("def");
                        Ok(vec![])
                    }
                }
            }
        };
        dbg!(comp);
    }
}
