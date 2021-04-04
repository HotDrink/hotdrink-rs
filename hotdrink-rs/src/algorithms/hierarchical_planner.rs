//! An implementation of a hierarchical planner based on `constraint_hierarchy_planner` from
//! [QuickPlan](https://dl.acm.org/doi/abs/10.1145/225540.225543?casa_token=bNBt7g-IvVQAAAAA:qHTNWG2wtiEUZXDGFOu2ooj8TGl5yJbKf3OiDmv1mnnuy6VdvrsZuAmcQZbtdjyn0MA4WALYavk).
//! This will create a solution graph for a [`Component`], where
//! a priority order for variables can be included.
//! The algorithm will attempt to avoid modifying variables with higher priorities.
//!
//! # Examples
//!
//! ```rust
//! # use hotdrink_rs::{component, ret, algorithms::hierarchical_planner::{hierarchical_planner, OwnedEnforcedConstraint}, Component};
//! let component: Component<i32> = component! {
//!     component Comp {
//!         let a: i32 = 0, b: i32 = 0, c: i32 = 0;
//!         constraint C {
//!             m1(a: &i32, b: &i32) -> [c] = ret![*a + *b];
//!             m2(b: &i32, c: &i32) -> [a] = ret![*c - *b];
//!             m3(c: &i32, a: &i32) -> [b] = ret![*c - *a];
//!         }
//!     }
//! };
//! assert_eq!(
//!     hierarchical_planner(&component, &[0, 1, 2]),
//!     Ok(vec![OwnedEnforcedConstraint::new(
//!         "C",
//!         component["C"]["m1"].clone()
//!     )])
//! );
//! ```
//!
//! [`Component`]: crate::Component

use super::{
    pruner::{create_var_to_constraint, prune},
    simple_planner::{simple_planner, EnforcedConstraint},
};
use crate::{
    algorithms::toposorter::toposort,
    data::traits::{ComponentSpec, ConstraintSpec, MethodSpec, PlanError},
};
use std::fmt::Debug;

/// Represents a type with input- and output-indices.
/// This can be used to represent Vertices in graphs.
pub trait Vertex {
    /// Returns the input-indices of this vertex.
    fn inputs(&self) -> &[usize];

    /// Returns the number of inputs of this vertex.
    fn n_inputs(&self) -> usize {
        self.inputs().len()
    }

    /// Returns the outputs-indices of this vertex.
    fn outputs(&self) -> &[usize];

    /// Returns the number of outputs of this vertex.
    fn n_outputs(&self) -> usize {
        self.outputs().len()
    }

    /// Creates a vertex that reads and writes to the same index.
    /// That is, if we create a vertex `v = stay(3)`,
    /// its only input is 3, and its only output is 3.
    fn stay(index: usize) -> Self;

    /// Returns true if the vertex reads from and writes to the same index.
    fn is_stay(&self) -> bool;
}

/// A constraint with name `name` that has been enforced with `method`.
/// This variation of `EnforcedConstraint` owns the name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OwnedEnforcedConstraint<M> {
    /// The name of enforced constraint.
    name: String,
    /// The method that enforces it.
    method: M,
}

impl<M> OwnedEnforcedConstraint<M> {
    /// Creates a new enforced constraint with name `name` that is enforced by `method`.
    pub fn new<S: Into<String>>(name: S, method: M) -> Self {
        Self {
            name: name.into(),
            method,
        }
    }

    /// Returns a reference to the name of the constraint.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the method that enforces the constraint.
    pub fn method(&self) -> &M {
        &self.method
    }
}

impl<'a, M: Clone> From<EnforcedConstraint<'a, M>> for OwnedEnforcedConstraint<M> {
    fn from(satisfied_constraint: EnforcedConstraint<'a, M>) -> Self {
        Self {
            name: satisfied_constraint.name().to_owned(),
            method: satisfied_constraint.method().to_owned(),
        }
    }
}

impl<M: Vertex> Vertex for OwnedEnforcedConstraint<M> {
    fn inputs(&self) -> &[usize] {
        self.method.inputs()
    }

    fn outputs(&self) -> &[usize] {
        self.method.outputs()
    }

    fn stay(index: usize) -> Self {
        Self {
            name: format!("stay({})", index),
            method: M::stay(index),
        }
    }

    fn is_stay(&self) -> bool {
        self.method.is_stay()
    }
}

/// A plan that consists of a `Vec` of `OwnedEnforcedConstraint`.
/// Each constraint must be enforced by a method for it to be a solution graph,
/// and the graph must also be a DAG.
pub type OwnedPlan<M> = Vec<OwnedEnforcedConstraint<M>>;

/// Take a component as input, as well as a ranking of variables to know which ones
/// should not be modified if possible. The leftmost variables will be prioritized.
///
/// This planner repeatedly calls the simple planner with different combinations of stay constraints.
/// The initial plan is just the one with no additional stay constraints to have a default.
/// Then try adding a stay constraint for the leftmost variable (highest priority), and see if the simple planner succeeds.
/// If it does, then keep the stay constraint and move on to the next variable.
/// If it does not succceed, then remove the stay constraint and move on to the next variable.
pub fn hierarchical_planner<T, M, C, Comp>(
    component: &Comp,
    ranking: &[usize],
) -> Result<OwnedPlan<M>, PlanError>
where
    M: MethodSpec<Arg = T> + Clone,
    C: ConstraintSpec<Method = M> + Debug + Clone,
    Comp: ComponentSpec<Constraint = C> + Clone,
{
    log::trace!("Calling hierarchical planner");
    // The initial solution with no stay constraints. If this fails, just return.
    let mut best_solution: Option<OwnedPlan<M>> = None;

    // Clone the component to be able to modify it
    let mut component = component.clone();
    // Lock variables that can't have a stay constraint
    let mut can_stay = vec![true; ranking.len()];
    // Create a map from variables to constraints
    let mut var_to_constraint = create_var_to_constraint(&component);

    // Try to find the best combination of stay constraints that works (lexicographic order)
    for &var_id in ranking {
        if !can_stay[var_id] {
            continue;
        }
        // Create a stay constraint
        let stay_method = M::stay(var_id);
        let stay_constraint = C::new(vec![stay_method]);
        component.push(stay_constraint);

        // If the constraint is a source in the solution graph, adding it is no issue.
        if let Some(bs) = &best_solution {
            let is_source = bs.iter().all(|m| !m.outputs().contains(&var_id));
            if is_source {
                var_to_constraint[var_id].insert(component.constraints().len() - 1);
                prune(
                    &mut var_to_constraint,
                    var_id,
                    &mut can_stay,
                    &mut component,
                );
                continue;
            }
        }

        log::trace!("Calling simple");
        // Check if this new solution works
        match simple_planner(&component) {
            Some(new_solution) => {
                var_to_constraint[var_id].insert(component.constraints().len() - 1);
                // Update best solution
                best_solution = Some(new_solution.into_iter().map(|sc| sc.into()).collect());
            }
            None => {
                // Can't satisfy this stay constraint, pop it.
                component.pop();
            }
        }

        // If the stay constraint could be added, then it will always be written to, and we can prune from it.
        // If the stay constraint could not be added, then this must be because the variable must be written to by
        // some other method.
        //
        // In either case, we can prune from the variable.
        prune(
            &mut var_to_constraint,
            var_id,
            &mut can_stay,
            &mut component,
        );
    }

    // Remove stay constraints
    let best_solution = best_solution
        .or_else(|| simple_planner(&component).map(|p| p.into_iter().map(|sc| sc.into()).collect()))
        .ok_or(PlanError::Overconstrained)?;
    let best_solution: Vec<OwnedEnforcedConstraint<_>> =
        best_solution.into_iter().filter(|m| !m.is_stay()).collect();
    let sorted = toposort(&best_solution, component.variables().len())
        .map(|v| v.into_iter().cloned().collect());

    sorted.ok_or(PlanError::Overconstrained)
}

#[cfg(test)]
mod tests {

    use super::{hierarchical_planner, OwnedEnforcedConstraint};
    use crate::{
        data::{
            component::Component,
            constraint::Constraint,
            method::Method,
            traits::{ComponentSpec, ConstraintSpec, MethodSpec},
        },
        ret,
    };
    use std::sync::Arc;

    #[test]
    fn hierarchical_planner_1() {
        let a_to_b = Method::new("a_to_b".to_string(), vec![0], vec![1], Arc::new(Ok));
        let b_to_a = Method::new("b_to_a".to_string(), vec![1], vec![0], Arc::new(Ok));
        let component: Component<&str> = Component::new(
            "foo".to_string(),
            vec!["a", "b"],
            vec![Constraint::new(vec![a_to_b.clone(), b_to_a.clone()])],
        );
        assert_eq!(
            hierarchical_planner::<&str, _, _, _>(&component, &[0, 1]),
            Ok(vec![OwnedEnforcedConstraint::new("", a_to_b)])
        );
        assert_eq!(
            hierarchical_planner::<&str, _, _, _>(&component, &[1, 0]),
            Ok(vec![OwnedEnforcedConstraint::new("", b_to_a)])
        );
    }

    #[test]
    fn hierarchical_planner_2() {
        let component: Component<i32> = crate::component! {
            component Comp {
                let a: i32 = 0, b: i32 = 0, c: i32 = 0;
                constraint C {
                    m1(a: &i32, b: &i32) -> [c] = ret![*a + *b];
                    m2(b: &i32, c: &i32) -> [a] = ret![*c - *b];
                    m3(c: &i32, a: &i32) -> [b] = ret![*c - *a];
                }
            }
        };
        assert_eq!(
            hierarchical_planner(&component, &[0, 1, 2]),
            Ok(vec![OwnedEnforcedConstraint::new(
                "C",
                component["C"]["m1"].clone()
            )])
        );
    }

    #[test]
    fn hierarchical_planner_two_way_chain() {
        let component = crate::dummy_component! {
            let a, b, c, d;
            constraint A {
                a1(a) -> [b];
                a2(b) -> [a];
            }
            constraint B {
                b1(b) -> [c];
                b2(c) -> [b];
            }
            constraint C {
                c1(c) -> [d];
                c2(d) -> [c];
            }
        };
        assert_eq!(
            hierarchical_planner(&component, &[0, 1, 2, 3]),
            Ok(vec![
                OwnedEnforcedConstraint::new("A", component["A"]["a1"].clone()),
                OwnedEnforcedConstraint::new("B", component["B"]["b1"].clone()),
                OwnedEnforcedConstraint::new("C", component["C"]["c1"].clone()),
            ])
        );
    }

    #[test]
    fn hierarchical_planner_ladder() {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .init();
        let component: Component<()> = crate::examples::components::ladder::ladder(12);
        pretty_assertions::assert_eq!(
            hierarchical_planner(&component, &(0..12).collect::<Vec<_>>()),
            Ok(vec![
                OwnedEnforcedConstraint::new("c0", component["c0"]["lower2"].clone()),
                OwnedEnforcedConstraint::new("c1", component["c1"]["upper2"].clone()),
                OwnedEnforcedConstraint::new("c2", component["c2"]["lower2"].clone()),
                OwnedEnforcedConstraint::new("c3", component["c3"]["upper2"].clone()),
                OwnedEnforcedConstraint::new("c4", component["c4"]["lower2"].clone()),
                OwnedEnforcedConstraint::new("c5", component["c5"]["upper2"].clone()),
                OwnedEnforcedConstraint::new("c6", component["c6"]["lower2"].clone()),
                OwnedEnforcedConstraint::new("c7", component["c7"]["upper2"].clone()),
                OwnedEnforcedConstraint::new("c8", component["c8"]["lower2"].clone()),
                OwnedEnforcedConstraint::new("c9", component["c9"]["upper2"].clone()),
            ])
        );
    }
}
