//! A macro for defining a closure that takes a vector as input, but allows for
//! using the parameters in the body as named variables.

/// Define a closure that takes a vector as input,
/// but turns the elements into the specified variables.
#[macro_export]
macro_rules! dyn_fn {
    (@define_refs $values:expr $(,)? ) => {{}};
    (@define_refs $values:expr, $name:ident: & $t:ty $(, $($rest:tt)* )? ) => {
        // Get reference and try to convert it
        let $name: &$t = $values.remove(0).try_into_ref()?;
        $crate::dyn_fn!(@define_refs $values $(, $($rest)*)?);
    };
    (@define_refs $values:expr, $name:ident: &mut $t:ty $(, $($rest:tt)* )? ) => {
        // Get reference and try to convert it
        let $name: &mut $t = $values.remove(0).try_into_mut()?;
        $crate::dyn_fn!(@define_refs $values $(, $($rest)*)?);
    };
    (fn ( $( $param:tt )* ) $e:block ) => {{
        use $crate::builders::MethodArg;
        #[allow(unused_mut, unused_variables)]
        |mut values: Vec<MethodArg<'_, _>>| -> Result<_, _> {
            $crate::dyn_fn!(@define_refs values, $( $param )*);
            $e
        }
    }}
}

#[cfg(test)]
mod tests {
    use crate::{builders::MethodArg, MethodResult};

    #[test]
    fn dyn_fn() {
        fn make() -> impl Fn(Vec<MethodArg<'_, i32>>) -> MethodResult<i32> {
            crate::dyn_fn!(fn(a: &i32, b: &mut i32) {
                Ok(vec![1, 2, 3])
            })
        }
        let f = make();
        let _ = f(vec![MethodArg::Ref(&3i32), MethodArg::MutRef(&mut 4)]);
    }
}
