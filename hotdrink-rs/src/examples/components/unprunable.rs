//! Functions for making unprunable constraint systems.

use crate::{
    examples::constraint_systems::factory::make_component,
    macros::{RawConstraint, RawMethod},
    model::Component,
};
use std::{fmt::Debug, sync::Arc};

use super::factory::ComponentFactory;

/// The root has one constraint with its children, there are three methods,
/// each one reads from one of them and writes to the two others.
pub fn unprunable<T>(n_variables: usize) -> Component<T>
where
    T: Debug + Clone + Default + 'static,
{
    let make_constraints: fn(&[String], &[String]) -> Vec<RawConstraint<T>> =
        |constraint_names, variable_names| {
            let n_variables = variable_names.len();
            let apply = Arc::new(|v: Vec<Arc<T>>| {
                let value = (*v[0]).clone();
                Ok(vec![Arc::new(value.clone()), Arc::new(value)])
            });
            let mut constraints = Vec::new();
            for current in 0..n_variables {
                let current_name = &variable_names[current];
                let left = 2 * current + 1;
                let right = 2 * current + 2;
                if left >= n_variables || right >= n_variables {
                    break;
                }
                let left_name = &variable_names[left];
                let right_name = &variable_names[right];
                constraints.push(RawConstraint::new(
                    &constraint_names[current],
                    vec![
                        RawMethod::new(
                            "down_left",
                            vec![current_name.as_str(), right_name.as_str()],
                            vec![left_name.as_str()],
                            apply.clone(),
                        ),
                        RawMethod::new(
                            "down_right",
                            vec![current_name.as_str(), left_name.as_str()],
                            vec![right_name.as_str()],
                            apply.clone(),
                        ),
                    ],
                ));
            }
            constraints
        };
    make_component(n_variables, make_constraints)
}

/// A component factory for creating unprunable components.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Unprunable;

impl ComponentFactory for Unprunable {
    fn name() -> &'static str {
        "unprunable"
    }
    fn build<T>(n_constraints: usize) -> Component<T>
    where
        T: Clone + Debug + Default + 'static,
    {
        let depth = (n_constraints as f64).log2();
        let n_variables = 2f64.powf(depth + 1.0);
        unprunable(n_variables as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::Unprunable;
    use crate::examples::components::factory::ComponentFactory;
    use crate::model::Component;
    use crate::planner::ComponentSpec;

    #[test]
    fn right_number_of_variables() {
        for n_constraints in 0..100 {
            let component: Component<()> = Unprunable::build(n_constraints);
            assert!(
                n_constraints.saturating_sub(1) <= component.n_constraints()
                    && component.n_constraints() <= n_constraints
            );
        }
    }
}
