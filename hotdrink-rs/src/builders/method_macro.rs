//! A macro that provides a DSL for constructing methods.

/// Introduce references to the specified parameters.
macro_rules! define_refs {
    ( $values:expr $(,)? ) => {{}};
    ( $values:expr, $name:ident: & $t:ty $(, $($rest:tt)* )? ) => {
        // Get reference and try to convert it
        let $name: &$t = $values.remove(0).try_into_ref()?;
        define_refs!($values $(, $($rest)*)?);
    };
    ( $values:expr, $name:ident: &mut $t:ty $(, $($rest:tt)* )? ) => {
        // Get reference and try to convert it
        let $name: &mut $t = $values.remove(0).try_into_mut::<$t>()?;
        define_refs!($values $(, $($rest)*)?);
    };
}

/// Turn a parameter list into a list of [`MethodInput`](crate::builders::method_builder::MethodInput).
macro_rules! make_params {
    () => {{ Vec::new() }};
    ( $name:ident: & $t:ty $(, $($rest:tt)* )? ) => {{
        use crate::builders::method_builder::MethodInput;
        let mut v = vec![MethodInput::make_ref(stringify!($name))];
        v.extend(make_params!( $( $( $rest )* )? ));
        v
    }};
    ( $name:ident: &mut $t:ty $(, $($rest:tt)* )? ) => {{
        use crate::builders::method_builder::MethodInput;
        let mut v = vec![MethodInput::make_mut_ref(stringify!($name))];
        v.extend(make_params!( $( $( $rest )* )? ));
        v
    }};
}

/// A macro for creating a `MethodBuilder`.
macro_rules! method {
    ( @impure $( $rest:tt )*) => { method!($($rest)*).pure(false) };
    (
        fn $method_name:ident ( $( $params:tt )* ) $( -> [ $( $output:ident ),* ] )? $e:block
    ) => {{
        use $crate::builders::MethodBuilder;
        MethodBuilder::new(stringify!($method_name))
            .inputs( make_params!( $( $params )* ) )
            .outputs( vec![ $( $( $crate::builders::MethodOutput::new(stringify!($output)) ),* )? ] )
            .apply(
                #[allow(unused_mut, unused_variables)]
                |mut values| {
                    define_refs!(values, $($params)*);
                    $e
            })
    }};
}

/// A macro for simplifying returning values from a method.
macro_rules! ret2 {
    ($($e:expr),*) => {{ Ok(vec![$($e.into()),*]) }}
}

#[cfg(test)]
mod tests {
    use crate::builders::MethodInput;

    #[test]
    fn make_params() {
        let _: Vec<MethodInput> = make_params!();
        assert_eq!(make_params!(a: &i32), vec![MethodInput::make_ref("a")]);
        assert_eq!(
            make_params!(a: &mut i32),
            vec![MethodInput::make_mut_ref("a")]
        );
        assert_eq!(
            make_params!(a: &i32, b: &mut i32),
            vec![MethodInput::make_ref("a"), MethodInput::make_mut_ref("b")]
        );

        let many = make_params!(
            a: &i32,
            b: &mut i32,
            c: &i32,
            d: &mut String,
            e: &std::collections::HashMap,
            f: &mut std::collections::HashMap
        );
        assert_eq!(
            many,
            vec![
                MethodInput::make_ref("a"),
                MethodInput::make_mut_ref("b"),
                MethodInput::make_ref("c"),
                MethodInput::make_mut_ref("d"),
                MethodInput::make_ref("e"),
                MethodInput::make_mut_ref("f")
            ]
        );
    }
}
