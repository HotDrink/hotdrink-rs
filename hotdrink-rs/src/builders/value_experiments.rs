//! Experiments regarding the innermost `Value`-wrapper around values.
//! As of now, there are only read-only references to values in the constraint system.
//! This means that modifications to large data-structures will have to make a copy.

use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

/// A value in the constraint system.
#[derive(Clone, Debug)]
pub enum Value<T> {
    /// A value that is only read.
    Ref(Arc<T>),
    /// A value that is mutated in-place.
    MutRef(Arc<RwLock<T>>),
}

impl<T> Value<T> {
    /// Returns a [`ValueReadGuard`] that provides an immutable reference to the inner value.
    pub fn read(&self) -> ValueReadGuard<'_, T> {
        match self {
            Value::Ref(value) => ValueReadGuard::Ref(value),
            Value::MutRef(value) => {
                ValueReadGuard::ReadGuard(value.read().expect("Lock was poisoned"))
            }
        }
    }

    /// Returns a [`ValueWriteGuard`] that provides a mutable reference to the inner value.
    pub fn write(&self) -> Option<ValueWriteGuard<'_, T>> {
        match self {
            Value::MutRef(value) => Some(ValueWriteGuard::WriteGuard(
                value.write().expect("Lock was poisoned"),
            )),
            Value::Ref(_) => None,
        }
    }

    /// Updates the inner value of the `Value`.
    /// This should not change its writability.
    pub fn update(&mut self, new_value: T) {
        match self {
            Value::Ref(_) => *self = Value::Ref(Arc::new(new_value)),
            Value::MutRef(ref mut old_value) => {
                let mut write_lock = old_value.write().expect("Lock was poisoned");
                *write_lock = new_value;
            }
        }
    }

    /// Returns true if the inner value can be mutated in-place.
    pub fn is_mutable(&self) -> bool {
        matches!(self, Value::MutRef(_))
    }
}

impl<T: Eq> PartialEq for Value<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_read_guard = self.read();
        let other_read_guard = other.read();
        let x: &T = &self_read_guard;
        let y: &T = &other_read_guard;
        x == y
    }
}

impl<T: Eq> Eq for Value<T> {}

/// RAII structure for getting read-access to the inner value of a [`Value`].
/// This struct is created by [`Value::read`].
#[derive(Debug)]
pub enum ValueReadGuard<'a, T> {
    /// A read-only reference to an immutable variable
    Ref(&'a Arc<T>),
    /// A read-only reference to a mutable variable
    ReadGuard(RwLockReadGuard<'a, T>),
}

impl<'a, T> Deref for ValueReadGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            ValueReadGuard::Ref(value) => value.deref(),
            ValueReadGuard::ReadGuard(value) => value.deref(),
        }
    }
}

/// RAII structure for getting write-access to the inner value of a [`Value`].
/// This struct is created by [`Value::write`].
#[derive(Debug)]
pub enum ValueWriteGuard<'a, T> {
    /// A write reference to a mutable variable.
    WriteGuard(RwLockWriteGuard<'a, T>),
}

impl<'a, T> Deref for ValueWriteGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            ValueWriteGuard::WriteGuard(value) => value.deref(),
        }
    }
}

impl<'a, T> DerefMut for ValueWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ValueWriteGuard::WriteGuard(value) => value.deref_mut(),
        }
    }
}

/// Declare variables as references or mutable references.
macro_rules! make_variables {
    (
            let $( $let_variable:ident = $let_value:expr ),*;
            mut $( $mut_variable:ident = $mut_value:expr ),*;
            $e:expr
    ) => {
        $(
            let $let_variable = {
                use $crate::builders::value_experiments::Value;
                Value::Ref(Arc::new($let_value))
            };
        )*
        $(
            let $mut_variable = {
                use std::sync::{Arc, RwLock};
                use $crate::builders::value_experiments::Value;
                Value::MutRef(Arc::new(RwLock::new($mut_value)))
            };
        )*
        $e
    };
}

/// Use variables as references or mutable references.
macro_rules! use_variables {
    (
        let $( $let_variable:ident ),*;
        mut $( $mut_variable:ident ),*;
        $e:expr
    ) => {{
        use std::ops::{Deref, DerefMut};
        $(
            let arc_or_read_guard = match $let_variable {
                Value::Ref(ref value) => Ok(value),
                Value::MutRef(ref value) => Err(value.read().unwrap()),
            };
            let $let_variable = match arc_or_read_guard {
                Ok(ref value) => value.deref(),
                Err(ref guard) => guard.deref(),
            };
        )*
        $(
            let mut opt_write_guard = match $mut_variable {
                Value::MutRef(ref value) => Some(value.write().unwrap()),
                Value::Ref(_) => None,
            };
            let $mut_variable = match opt_write_guard {
                Some(ref mut guard) => guard.deref_mut(),
                None => panic!("Read only"),
            };
        )*
        $e
    }}
}

#[cfg(test)]
mod tests {
    use super::Value;
    use std::sync::{Arc, Mutex};

    #[test]
    fn mut_ref_from_mutex() {
        let x = Arc::new(Mutex::new(3));
        {
            let _x_ref: &i32 = &x.lock().unwrap();
        }

        {
            let _x_mut_ref: &mut i32 = &mut x.lock().unwrap();
        }
    }

    #[test]
    fn some_mut() {
        make_variables! {
            let b = 0;
            mut a = 0, c = 0;
            {
                assert!(matches!(a, Value::MutRef(_)));
                assert!(matches!(b, Value::Ref(_)));
                assert!(matches!(c, Value::MutRef(_)));

                use_variables! {
                    let a, b;
                    mut c;
                    {
                        let a: &i32 = a;
                        let b: &i32 = b;
                        let c: &mut i32 = c;
                        dbg!(&c);
                        *c += 3;
                        dbg!(&a);
                        dbg!(&b);
                        dbg!(&c);
                    }
                }
            }
        };
    }
}
