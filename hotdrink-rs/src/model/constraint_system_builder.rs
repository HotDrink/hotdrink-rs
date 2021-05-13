use super::{Component, ConstraintSystem};
use derivative::Derivative;

/// A builder of [`ConstraintSystem`]s.
#[derive(Derivative)]
#[derivative(Clone, Debug, Default(bound = ""), PartialEq, Eq)]
pub struct ConstraintSystemBuilder<T> {
    components: Vec<Component<T>>,
}

impl<T> ConstraintSystemBuilder<T> {
    /// Adds a single component.
    pub fn component(&mut self, component: Component<T>) -> &mut Self {
        self.components.push(component);
        self
    }

    /// Builds a [`ConstraintSystem`].
    pub fn build(&mut self) -> ConstraintSystem<T> {
        let mut cs = ConstraintSystem::new();
        for c in &self.components {
            cs.add_component(c.clone());
        }
        cs
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        component, component_type,
        model::{Component, ConstraintSystem},
    };

    #[test]
    pub fn build_constraint_system() {
        component_type! {
            #[derive(Clone, Debug, PartialEq)]
            enum I32OrString {
                i32,
                String
            }
        }
        let a: Component<I32OrString> = component! {
            component Foo { let a: i32 = 0, b: i32 = 0; }
        };
        let b = component! {
            component Foo { let x: i32 = 0, y: String = ""; }
        };
        let actual = ConstraintSystemBuilder::default()
            .component(a.clone())
            .component(b.clone())
            .build();

        let mut expected = ConstraintSystem::new();
        expected.add_component(a);
        expected.add_component(b);
        assert_eq!(actual, expected);
    }
}
