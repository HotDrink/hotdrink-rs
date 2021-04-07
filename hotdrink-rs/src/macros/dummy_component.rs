//! A macro for quickly and easily defining a [`Component<()>`](crate::model::Component).

/// A macro for quickly and easily defining a [`Component<()>`](crate::model::Component).
/// This is useful for testing and benchmarks.
#[macro_export]
macro_rules! dummy_component {
    (
        // Declarations
        let $($variable_name:ident),*;
        $(
            // Constraints
            constraint $constraint_name:ident {
                $(
                    // Methods
                    $method_name:ident( $($input:ident),* ) -> [ $($output:ident),* ];
                )*
            }
        )*
    ) => {
        $crate::component! {
            component DummyComponent {
                let $($variable_name: () = ()),*;
                $(
                    constraint $constraint_name {
                        $(
                            $method_name( $( $input: &() ),* ) -> [ $( $output ),* ] = ret![ $( *$input ),* ];
                        )*
                    }
                )*
            }
        }
    }
}
