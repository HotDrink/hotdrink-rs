//! A macro that provides a DSL for constructing components.

/// A macro for declaratively making components.
macro_rules! build_component {
    (@value_or_default: $t:ty ) => {{ <$t>::default() }};
    (@value_or_default: $t:ty = $value:expr) => {{ $value }};
    (
        component $component_name:ident {
            $( let $( $let_variable:ident : $let_variable_type:ty $( = $let_value:expr )? ),* ; )?
            $( mut $( $mut_variable:ident : $mut_variable_type:ty $( = $mut_value:expr )? ),* ; )?
            $(
                constraint $constraint_name:ident {
                    $(
                        $(@$impure:ident)? fn $method_name:ident
                            ( $( $param:tt )* )
                            $( -> [ $( $output:ident ),* ] )?
                            $e:block
                    )*
                }
            )*
        }
    ) => {{
        #[allow(unused_mut)]
        let mut component_builder = $crate::builders::ComponentBuilder::new(stringify!($component_name));

        // Add immutable variables
        $( $(
            let let_value: $let_variable_type = (build_component!(@value_or_default: $let_variable_type $( = $let_value )?)).into();
            component_builder = component_builder.variable(stringify!($let_variable), let_value.into());
        )* )?

        // Add mutable variables
        $( $(
            let mut_value: $mut_variable_type = (build_component!(@value_or_default: $mut_variable_type $( = $mut_value )?)).into();
            component_builder = component_builder.variable_mut(stringify!($mut_variable), mut_value.into());
        )* )?

        // Add constraints
        component_builder $(
            .constraint({
                #[allow(unused_mut)]
                $crate::builders::ConstraintBuilder::new(stringify!($constraint_name)) $(
                    .method(
                        method!(
                            $(@$impure)?
                            fn $method_name
                                ( $( $param )* )
                                $( -> [ $( $output ),* ] )?
                                $e
                        )
                    )
                )*
            })
        )*
    }};
}

#[cfg(test)]
mod tests {
    use crate::builders::ComponentBuilder;

    #[test]
    fn build_component() {
        sum_type! {
            #[derive(Debug)]
            enum Foo {
                i32,
                f64,
                String,
            }
        }
        let comp: ComponentBuilder<Foo> = build_component! {
            component Component {
                let x: i32 = 0, y: i32, z: f64;
                mut s: String;
                constraint Constraint {
                    fn m() { Ok(vec![]) }
                    fn add(x: &i32, y: &i32) { Ok(vec![]) }
                    @impure
                    fn append(s: &mut String) {
                        s.push_str("def");
                        Ok(vec![])
                    }
                }
            }
        };
        dbg!(comp);
    }
}
