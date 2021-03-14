use super::{hierarchical_planner::Vertex, toposorter::toposort};
use crate::data::traits::{ComponentLike, ConstraintLike, MethodLike};
use itertools::Itertools;
use std::{collections::VecDeque, fmt::Debug};

#[derive(Debug, Clone, Default)]
pub struct VariableRefCounter {
    referencing_constraints: Vec<usize>,
}

impl VariableRefCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_free(&self) -> bool {
        self.referencing_constraints.len() == 1
    }

    pub fn add_reference(&mut self, index: usize) {
        self.referencing_constraints.push(index);
    }

    pub fn remove_reference(&mut self, index: usize) {
        self.referencing_constraints.retain(|ci| &index != ci);
    }

    pub fn referencing_constraints(&self) -> &[usize] {
        &self.referencing_constraints
    }

    fn count_variable_refs<Comp: ComponentLike>(component: &Comp) -> Vec<Self> {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SatisfiedConstraint<'a, M> {
    name: &'a str,
    method: &'a M,
}

impl<'a, M> SatisfiedConstraint<'a, M> {
    pub fn new(name: &'a str, method: &'a M) -> Self {
        Self { name, method }
    }
    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn method(&self) -> &'a M {
        self.method
    }
}

impl<'a, M: Vertex> Vertex for SatisfiedConstraint<'a, M> {
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

pub type Plan<'a, M> = Vec<SatisfiedConstraint<'a, M>>;
// pub type Plan<'a, M> = Vec<(&'a str, &'a M)>;

#[allow(clippy::needless_lifetimes)]
pub fn simple_planner<'a, M, C, Comp>(component: &'a Comp) -> Option<Plan<'a, M>>
where
    M: MethodLike,
    C: ConstraintLike<Method = M> + 'a + Debug,
    Comp: ComponentLike<Constraint = C>,
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
            // Find all methods that write to free variables,
            // sorted by their number of outputs.
            let mut free_methods = constraint
                .methods()
                .iter()
                .filter(|m| m.outputs().contains(&idx))
                .filter(|m| m.outputs().iter().all(|o| variables[*o].is_free()))
                .sorted_by_key(|m| m.outputs().len());

            if let Some(m) = free_methods.next() {
                // Add this method to the plan
                plan.push(SatisfiedConstraint::new(constraint.name(), m));
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

#[allow(clippy::needless_lifetimes)]
pub fn new_simple_planner_toposort<'a, M, C, Comp>(
    component: &'a Comp,
) -> Option<Vec<SatisfiedConstraint<'a, M>>>
where
    M: MethodLike + Clone,
    C: ConstraintLike<Method = M> + 'a + Debug,
    Comp: ComponentLike<Constraint = C>,
{
    let plan: Vec<SatisfiedConstraint<'a, M>> = simple_planner(component)?;
    let sorted_plan: Vec<&'_ SatisfiedConstraint<'a, M>> =
        toposort(&plan, component.n_variables())?;
    Some(sorted_plan.into_iter().cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::SatisfiedConstraint;
    use crate::{
        algorithms::{
            simple_planner::{new_simple_planner_toposort, simple_planner},
            toposorter::toposort,
        },
        data::{component::Component, traits::ComponentLike},
    };
    use crate::{data::constraint_system::ConstraintSystem, ret};

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
        assert_eq!(plan, Some(vec![SatisfiedConstraint::new("C", c)]));
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
        let plan = new_simple_planner_toposort(&comp);
        assert_eq!(
            plan,
            Some(vec![
                SatisfiedConstraint::new("Product", p1),
                SatisfiedConstraint::new("Sum", s1)
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
                SatisfiedConstraint::new("d_to_e", &comp["d_to_e"]["d_to_e"]),
                SatisfiedConstraint::new("c_to_d", &comp["c_to_d"]["c_to_d"]),
                SatisfiedConstraint::new("b_to_c", &comp["b_to_c"]["b_to_c"]),
                SatisfiedConstraint::new("a_to_b", &comp["a_to_b"]["a_to_b"]),
            ])
        );
        let plan = plan.unwrap();
        let sorted = toposort(&plan, comp.n_variables());
        assert_eq!(
            sorted,
            Some(vec![
                &SatisfiedConstraint::new("a_to_b", &comp["a_to_b"]["a_to_b"]),
                &SatisfiedConstraint::new("b_to_c", &comp["b_to_c"]["b_to_c"]),
                &SatisfiedConstraint::new("c_to_d", &comp["c_to_d"]["c_to_d"]),
                &SatisfiedConstraint::new("d_to_e", &comp["d_to_e"]["d_to_e"]),
            ])
        );
    }

    #[test]
    pub fn big_linear() {
        let linear_cs: ConstraintSystem<()> =
            crate::examples::constraint_systems::linear_oneway(1, 1000);
        let linear_comp = linear_cs.get_component("0");
        let _ = simple_planner(linear_comp);
    }

    #[test]
    pub fn big_tree() {
        let cs: ConstraintSystem<()> =
            crate::examples::constraint_systems::multioutput_singleway(1, 10);
        let comp = cs.get_component("0");
        let _ = simple_planner(comp);
    }

    #[test]
    pub fn big_dense() {
        let cs: ConstraintSystem<()> = crate::examples::constraint_systems::make_dense_cs(1, 100);
        let comp = cs.get_component("0");
        let _ = simple_planner(comp);
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
                SatisfiedConstraint::new("B", &comp["B"]["b1"]),
                SatisfiedConstraint::new("A", &comp["A"]["a1"])
            ])
        );
    }

    #[test]
    pub fn selected_free_method_must_write_to_variable() {
        let comp: Component<i32> = crate::component! {
            component Comp {
                let _a: i32 = 0;
                constraint A {
                    a1(_a: &i32) -> [] = ret![];
                }
            }
        };
        let plan = simple_planner(&comp);
        assert_eq!(plan, None,);
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    pub fn linear_bench(b: &mut Bencher) {
        let cs: ConstraintSystem<()> =
            crate::examples::constraint_systems::linear::linear_oneway(1, 400);
        let comp = cs.get_component("0");
        b.iter(|| simple_planner(comp));
    }

    #[bench]
    pub fn tree_bench(b: &mut Bencher) {
        let cs: ConstraintSystem<()> =
            crate::examples::constraint_systems::tree::multioutput_singleway(1, 400);
        let comp = cs.get_component("0");
        b.iter(|| simple_planner(comp));
    }
}
