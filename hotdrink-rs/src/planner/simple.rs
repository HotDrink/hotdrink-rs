//! An implementation of a simple planner based on the `multi_output_planner` from
//! [QuickPlan](https://dl.acm.org/doi/abs/10.1145/225540.225543?casa_token=bNBt7g-IvVQAAAAA:qHTNWG2wtiEUZXDGFOu2ooj8TGl5yJbKf3OiDmv1mnnuy6VdvrsZuAmcQZbtdjyn0MA4WALYavk).
//! Given a component, it will find one method per constraint to enforce it, such that the methods and the variables they
//! read from and write to form a directed acyclic graph.

use super::{hierarchical::Vertex, toposorter::toposort};
use crate::planner::{ComponentSpec, ConstraintSpec, MethodSpec};
use std::{collections::VecDeque, fmt::Debug};

/// Maintains a list of constraints that reference this variable.
#[derive(Debug, Clone, Default)]
pub struct VariableRefCounter {
    referencing_constraints: Vec<usize>,
}

impl VariableRefCounter {
    /// Creates a new `VariableRefCounter` with no references.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if the variable is free.
    /// That is, it only has a single constraint referencing it.
    pub fn is_free(&self) -> bool {
        self.referencing_constraints.len() == 1
    }

    /// Adds a reference to a specific constraint to this variable.
    pub fn add_reference(&mut self, index: usize) {
        self.referencing_constraints.push(index);
    }

    /// Removes a reference to a specific constraint from this variable.
    pub fn remove_reference(&mut self, index: usize) {
        self.referencing_constraints.retain(|ci| &index != ci);
    }

    /// Returns the list of constraints that reference this variable.
    pub fn referencing_constraints(&self) -> &[usize] {
        &self.referencing_constraints
    }

    /// Stores references for all variables used in a component.
    fn count_variable_refs<Comp: ComponentSpec>(component: &Comp) -> Vec<Self> {
        let n_variables = component.n_variables();
        let constraints = &component.constraints();
        let mut variables = vec![VariableRefCounter::new(); n_variables];
        for (ci, constraint) in constraints.iter().enumerate() {
            // Count the constraints that reference each variable
            for var_id in constraint.variables() {
                let variable = &mut variables[*var_id];
                variable.add_reference(ci);
            }
        }
        variables
    }
}

/// A constraint with a [`&str`] `name` that has been enforced with `method`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EnforcedConstraint<'a, M> {
    /// The name of enforced constraint.
    name: &'a str,
    /// The method that enforces it.
    method: &'a M,
}

impl<'a, M> EnforcedConstraint<'a, M> {
    /// Constructs a new `EnforcedConstraint` that is enforced by `method`.
    pub fn new(name: &'a str, method: &'a M) -> Self {
        Self { name, method }
    }
    /// Returns the name of the enforced constraint.
    pub fn name(&self) -> &'a str {
        self.name
    }
    /// Returns the method that enforces the constraint.
    pub fn method(&self) -> &'a M {
        self.method
    }
}

impl<'a, M: Vertex> Vertex for EnforcedConstraint<'a, M> {
    fn inputs(&self) -> &[usize] {
        self.method.inputs()
    }

    fn outputs(&self) -> &[usize] {
        self.method.outputs()
    }

    fn stay(_: usize) -> Self {
        unimplemented!()
    }

    fn is_stay(&self) -> bool {
        self.method.is_stay()
    }
}

/// A plan that consists of a [`Vec`] of [`EnforcedConstraint`].
/// Each constraint must be enforced by a method for it to be a solution graph,
/// and the graph must also be a DAG.
pub type Plan<'a, M> = Vec<EnforcedConstraint<'a, M>>;

/// Attempts to construct a plan from something that implements [`ComponentSpec`].
///
/// # Examples
///
/// ```rust
/// # use hotdrink_rs::{component, ret, planner::{simple_planner, EnforcedConstraint}, model::Method};
///
/// // Construct a component
/// let component = component! {
///     component Component {
///         let x: i32 = 0, y: i32 = 0;
///         constraint Eq {
///           m1(x: &i32) -> [y] = ret![*x];
///           m2(y: &i32) -> [x] = ret![*y];
///         }
///     }
/// };
///
/// // Run simple planner
/// let plan: Option<Vec<EnforcedConstraint<Method<i32>>>> = simple_planner(&component);
///
/// assert_eq!(plan, Some(vec![EnforcedConstraint::new("Eq", &component["Eq"]["m2"])]));
/// ```
#[allow(clippy::needless_lifetimes)]
pub fn simple_planner<'a, M, C, Comp>(component: &'a Comp) -> Option<Plan<'a, M>>
where
    M: MethodSpec,
    C: ConstraintSpec<Method = M> + 'a + Debug,
    Comp: ComponentSpec<Constraint = C>,
{
    let mut plan = Vec::with_capacity(component.constraints().len());
    let n_variables = component.n_variables();
    let constraints = component.constraints();
    let mut remaining_constraints = constraints.len();

    // Find the total use-count for each variable (n where n number of constraints)
    let mut variables = VariableRefCounter::count_variable_refs(component);

    // Add initial interesting variables
    let mut potentially_free_variables = VecDeque::with_capacity(n_variables);
    for (idx, variable) in variables.iter_mut().enumerate() {
        if variable.is_free() {
            potentially_free_variables.push_back(idx);
        }
    }

    while remaining_constraints != 0 {
        // Pick the first current interesting variable
        let idx = potentially_free_variables.pop_front()?;
        let interesting_variable = &variables[idx];

        // May have become uninteresting since we added it.
        // If it were interesting at some point, but is no more, then
        // the number of references must be 0.
        if !interesting_variable.is_free() {
            continue;
        }

        // Should only be one, otherwise it should not have been interesting.
        assert_eq!(interesting_variable.referencing_constraints.len(), 1);

        // Get the constraints with this variable, should only be one
        let referencing_constraints = interesting_variable.referencing_constraints().to_owned();
        for ci in referencing_constraints {
            let constraint = &constraints[ci];

            // Find best method candidate
            let mut free_method: Option<&M> = None;
            for m in constraint.methods() {
                let outputs = m.outputs();
                let contains_variable = outputs.contains(&idx);
                let all_outputs_are_free = outputs.iter().all(|o| variables[*o].is_free());
                if contains_variable && all_outputs_are_free {
                    if let Some(other) = free_method {
                        if m.n_outputs() < other.n_outputs() {
                            free_method = Some(m);
                        }
                    } else {
                        free_method = Some(m);
                    }
                }
            }

            if let Some(m) = free_method {
                // Add this method to the plan
                plan.push(EnforcedConstraint::new(constraint.name(), m));
                remaining_constraints -= 1;

                // Remove all references to this constraint
                for vi in constraint.variables() {
                    let variable = &mut variables[*vi];
                    variable.remove_reference(ci);
                    if variable.is_free() {
                        potentially_free_variables.push_back(*vi);
                    }
                }
            }
        }
    }

    Some(plan)
}

/// Runs the [`simple_planner`], and then topologically sorts the resulting plan.
/// If successful, this plan can then be run to enforce all the constraints in the input-component.
#[allow(clippy::needless_lifetimes)]
pub fn simple_planner_toposort<'a, M, C, Comp>(
    component: &'a Comp,
) -> Option<Vec<EnforcedConstraint<'a, M>>>
where
    M: MethodSpec + Clone,
    C: ConstraintSpec<Method = M> + 'a + Debug,
    Comp: ComponentSpec<Constraint = C>,
{
    let plan: Vec<EnforcedConstraint<'a, M>> = simple_planner(component)?;
    let sorted_plan: Vec<&'_ EnforcedConstraint<'a, M>> = toposort(&plan, component.n_variables())?;
    Some(sorted_plan.into_iter().cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::EnforcedConstraint;
    use crate::planner::{
        simple::{simple_planner, simple_planner_toposort},
        toposorter::toposort,
        ComponentSpec,
    };
    use crate::{
        model::{Component, ConstraintSystem},
        ret,
    };

    #[test]
    fn empty_component_gives_empty_plan() {
        let comp: Component<()> = crate::component! {
            component Comp {
                let a: () = ();
            }
        };
        let plan = simple_planner(&comp);
        assert_eq!(plan, Some(vec![]));
    }

    #[test]
    fn one_method_gives_single_method_plan() {
        let comp = crate::component! {
            component Comp {
                let a: () = ();
                constraint C {
                    c(a: &()) -> [a] = ret![*a];
                }
            }
        };
        let c = &comp["C"]["c"];
        let plan = simple_planner(&comp);
        assert_eq!(plan, Some(vec![EnforcedConstraint::new("C", c)]));
    }

    #[test]
    fn sum_product() {
        let comp: Component<i32> = crate::component! {
            component Comp {
                let a: i32 = 0, b: i32 = 0, c: i32 = 0, d: i32 = 0;

                constraint Sum {
                    s1(a: &i32, b: &i32) -> [c] = ret![a + b];
                    s2(b: &i32, c: &i32) -> [a] = ret![b + c];
                    s3(c: &i32, a: &i32) -> [b] = ret![c + a];
                }

                constraint Product {
                    p1(a: &i32, b: &i32) -> [d] = ret![a * b];
                    p2(b: &i32, d: &i32) -> [a] = ret![d / b];
                    p3(d: &i32, a: &i32) -> [b] = ret![d / a];
                }
            }
        };
        let p1 = &comp["Product"]["p1"];
        let s1 = &comp["Sum"]["s1"];
        let plan = simple_planner_toposort(&comp);
        assert_eq!(
            plan,
            Some(vec![
                EnforcedConstraint::new("Product", p1),
                EnforcedConstraint::new("Sum", s1)
            ])
        );
    }

    #[test]
    fn small_linear() {
        let comp: Component<i32> = crate::component! {
            component Comp {
                let a: i32 = 0, b: i32 = 0, c: i32 = 0, d: i32 = 0, e: i32 = 0;
                constraint a_to_b {
                    a_to_b(a: &i32) -> [b] = ret![*a];
                }
                constraint b_to_c {
                    b_to_c(b: &i32) -> [c] = ret![*b];
                }
                constraint c_to_d {
                    c_to_d(c: &i32) -> [d] = ret![*c];
                }
                constraint d_to_e {
                    d_to_e(d: &i32) -> [e] = ret![*d];
                }
            }
        };
        let plan = simple_planner(&comp);
        assert_eq!(
            &plan,
            &Some(vec![
                EnforcedConstraint::new("d_to_e", &comp["d_to_e"]["d_to_e"]),
                EnforcedConstraint::new("c_to_d", &comp["c_to_d"]["c_to_d"]),
                EnforcedConstraint::new("b_to_c", &comp["b_to_c"]["b_to_c"]),
                EnforcedConstraint::new("a_to_b", &comp["a_to_b"]["a_to_b"]),
            ])
        );
        let plan = plan.unwrap();
        let sorted = toposort(&plan, comp.n_variables());
        assert_eq!(
            sorted,
            Some(vec![
                &EnforcedConstraint::new("a_to_b", &comp["a_to_b"]["a_to_b"]),
                &EnforcedConstraint::new("b_to_c", &comp["b_to_c"]["b_to_c"]),
                &EnforcedConstraint::new("c_to_d", &comp["c_to_d"]["c_to_d"]),
                &EnforcedConstraint::new("d_to_e", &comp["d_to_e"]["d_to_e"]),
            ])
        );
    }

    #[test]
    pub fn big_linear() {
        let linear_cs: ConstraintSystem<()> =
            crate::examples::constraint_systems::linear_oneway(1, 1000);
        let linear_comp = linear_cs.component("0").unwrap();
        assert!(simple_planner(linear_comp).is_some())
    }

    #[test]
    pub fn big_tree() {
        let cs: ConstraintSystem<()> =
            crate::examples::constraint_systems::multioutput_singleway(1, 10);
        let comp = cs.component("0").unwrap();
        assert!(simple_planner(comp).is_some());
    }

    #[test]
    pub fn big_dense() {
        let cs: ConstraintSystem<()> = crate::examples::constraint_systems::make_dense_cs(1, 100);
        let comp = cs.component("0").unwrap();
        assert!(simple_planner(comp).is_some());
    }

    #[test]
    pub fn selects_method_where_all_outputs_are_free_first() {
        let comp: Component<i32> = crate::component! {
            component Comp {
                let a: i32 = 0, b: i32 = 0, c: i32 = 0, d: i32 = 0;
                constraint A {
                    a1(b: &i32) -> [a, c] = ret![*b, *b];
                }
                constraint B {
                    b1(c: &i32) -> [d] = ret![*c];
                }
            }
        };
        let plan = simple_planner(&comp);
        assert_eq!(
            plan,
            Some(vec![
                EnforcedConstraint::new("B", &comp["B"]["b1"]),
                EnforcedConstraint::new("A", &comp["A"]["a1"])
            ])
        );
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    pub fn linear_bench(b: &mut Bencher) {
        let cs: ConstraintSystem<()> =
            crate::examples::constraint_systems::linear::linear_oneway(1, 400);
        let comp = cs.component("0").unwrap();
        b.iter(|| simple_planner(comp));
    }

    #[bench]
    pub fn tree_bench(b: &mut Bencher) {
        let cs: ConstraintSystem<()> =
            crate::examples::constraint_systems::tree::multioutput_singleway(1, 400);
        let comp = cs.component("0").unwrap();
        b.iter(|| simple_planner(comp));
    }
}
