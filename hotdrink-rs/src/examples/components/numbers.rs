use crate::data::component::Component;
use crate::{component, ret};
use std::ops::{Add, Div, Mul, Sub};

/// A predefined component with three numeric variables a, b, and c that satisfy the constraint `a + b = c`.
pub fn sum<T>() -> Component<T>
where
    T: Add<Output = T> + Sub<Output = T> + Default + Copy,
{
    let v = T::default();
    component! {
        component Component {
            let a: T = v, b: T = v, c: T = v;
            constraint Sum {
                abc(a: &T, b: &T) -> [c] = ret![*a + *b];
                acb(a: &T, c: &T) -> [b] = ret![*c - *a];
                bca(b: &T, c: &T) -> [a] = ret![*c - *b];
            }
        }
    }
}

/// A predefined component with three numeric variables a, b, and c that satisfy the constraint `a * b = c`.
/// This one also requires that the default values are provided to avoid the `Default` restriction.
pub fn product_with_defaults<T>(a: T, b: T, c: T) -> Component<T>
where
    T: Mul<Output = T> + Div<Output = T> + Copy,
{
    component! {
        component comp {
            let a: T = a, b: T = b, c: T = c;
            constraint sum {
                abc(a: &T, b: &T) -> [c] = ret![*a * *b];
                acb(a: &T, c: &T) -> [b] = ret![*c / *a];
                bca(b: &T, c: &T) -> [a] = ret![*c / *b];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{product_with_defaults, sum};

    #[test]
    #[ignore = "This test is just to make sure it compiles"]
    fn it_compiles() {
        let _ = sum::<i32>();
        let _ = sum::<f32>();
        let _ = product_with_defaults::<u32>(3, 4, 12);
        let _ = product_with_defaults::<f64>(3.0, 4.0, 12.0);
    }
}
