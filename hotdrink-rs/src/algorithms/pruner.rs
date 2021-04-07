//! An algorithm for "locking in" methods of constraints that must be selected,
//! and removing those that can not be selected.
//! This can often make planning a lot faster by shrinking the constraint graph.

use crate::algorithms::hierarchical_planner::Vertex;
use crate::model::{ComponentSpec, ConstraintSpec, MethodSpec};
use std::collections::HashSet;

/// Create a map from variables to all components they are used in.
pub fn create_var_to_constraint(component: &impl ComponentSpec) -> Vec<HashSet<usize>> {
    let mut var_to_constraint = vec![HashSet::new(); component.n_variables()];
    for (ci, constraint) in component.constraints().iter().enumerate() {
        for &vi in constraint.variables() {
            var_to_constraint[vi].insert(ci);
        }
    }
    var_to_constraint
}

/// If it is verified that the given variable can have a stay constraint added,
/// then it may lock other methods near it, which may cascade through the system.
///
/// # Examples
///
/// Given a system with the following constraints
/// a <-> b <-> c
/// If we successfully solve the system with a stay constraint on A,
/// we can remove all other methods that write to a to get
/// a -> b <-> c
/// We then only have one potential method left to pick,
/// meaning we also know which method must write to b.
/// This means that we can remove other writes to b and get
/// a -> b -> c
/// And again, b only has one method left to pick, providing us with a write to c.
/// This leaves us with the following information: Two of the methods can be pruned,
/// and b and c can not be written to by anything else, even methods in stay constraints.
pub fn prune<C>(
    var_to_constraints: &mut [HashSet<usize>],
    start: usize,
    can_stay: &mut [bool],
    component: &mut C,
) where
    C: ComponentSpec,
{
    // Lock in variables by DFS-ing from the stay constraint added
    let mut visited = vec![false; component.n_variables()];
    let mut are_written_to = vec![start];
    while let Some(current_idx) = are_written_to.pop() {
        visited[current_idx] = true;
        can_stay[current_idx] = false;

        // Find the unique writer for this variable
        let mut unique_writer = None;
        for ci in &var_to_constraints[current_idx] {
            let constraint = &component.constraints_mut()[*ci];
            if let [m] = constraint.methods() {
                if m.outputs().contains(&current_idx) {
                    unique_writer = Some(ci);
                    break;
                }
            }
        }

        // If a unique writer exists
        if let Some(&uwci) = unique_writer {
            // Go through all connected constraints
            for &ci in &var_to_constraints[current_idx] {
                let constraint = &mut component.constraints_mut()[ci];
                // Except unique writer, and
                if ci != uwci {
                    // Remove methods in the other constraints that write to this variable
                    let to_remove: Vec<_> = constraint
                        .methods()
                        .iter()
                        .filter(|m| m.outputs().contains(&current_idx))
                        .map(|m| m.name().to_owned())
                        .collect();
                    for m in to_remove {
                        constraint.remove_method(&m);
                    }
                }
            }

            // For each enforced constraint (only 1 available method),
            // visit all the outputs that are now definitely written to.
            for ci in &var_to_constraints[current_idx] {
                let constraint = &mut component.constraints_mut()[*ci];
                let methods = constraint.methods();
                if let [m] = methods {
                    for &o in m.outputs() {
                        if !visited[o] {
                            are_written_to.push(o);
                        }
                    }
                }
            }

            // Update var_to_constraints to only contain the unique writer
            var_to_constraints[current_idx].clear();
            var_to_constraints[current_idx].insert(uwci);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{create_var_to_constraint, prune};
    use crate::{dummy_component, model::ComponentSpec, ret};

    #[test]
    fn prune_one_way_chain_should_do_nothing() {
        let mut component = dummy_component! {
            let a, b, c, d;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A {
                a1(a) -> [b];
            }
            constraint B {
                b1(b) -> [c];
            }
            constraint C {
                c1(c) -> [d];
            }
        };

        // Run prune
        let mut can_stay = vec![true; component.n_variables()];
        prune(
            &mut create_var_to_constraint(&component),
            0,
            &mut can_stay,
            &mut component,
        );

        let expected = dummy_component! {
            let a, b, c, d;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A { a1(a) -> [b]; }
            constraint B { b1(b) -> [c]; }
            constraint C { c1(c) -> [d]; }
        };

        assert_eq!(component, expected);
        assert_eq!(can_stay, vec![false, false, false, false]);
    }

    #[test]
    fn prune_two_way_chain() {
        let mut component = dummy_component! {
            let a, b, c, d;
            constraint StayA {
                id(a) -> [a];
            }
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

        // Run prune
        let mut can_stay = vec![true; component.n_variables()];
        prune(
            &mut create_var_to_constraint(&component),
            0,
            &mut can_stay,
            &mut component,
        );

        let expected = dummy_component! {
            let a, b, c, d;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A { a1(a) -> [b]; }
            constraint B { b1(b) -> [c]; }
            constraint C { c1(c) -> [d]; }
        };

        assert_eq!(component, expected);
        assert_eq!(can_stay, vec![false, false, false, false]);
    }

    #[test]
    fn prune_one_way_multi_output() {
        let mut component = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A {
                a_to_bc(a) -> [b, c];
            }
        };

        // Run prune
        let mut can_stay = vec![true; component.n_variables()];
        prune(
            &mut create_var_to_constraint(&component),
            0,
            &mut can_stay,
            &mut component,
        );

        let expected = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A { a_to_bc(a) -> [b, c]; }
        };

        assert_eq!(component, expected);
        assert_eq!(can_stay, vec![false, false, false]);
    }

    #[test]
    fn prune_two_way_multi_output() {
        let mut component = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A {
                a_to_bc(a) -> [b, c];
                b_to_ac(b) -> [a, c];
            }
        };

        // Run prune
        let mut can_stay = vec![true; component.n_variables()];
        prune(
            &mut create_var_to_constraint(&component),
            0,
            &mut can_stay,
            &mut component,
        );

        let expected = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A { a_to_bc(a) -> [b, c]; }
        };

        assert_eq!(component, expected);
        assert_eq!(can_stay, vec![false, false, false]);
    }

    #[test]
    fn prune_three_way_multi_output() {
        let mut component = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A {
                a_to_bc(a) -> [b, c];
                b_to_ac(b) -> [a, c];
                c_to_ab(c) -> [a, b];
            }
        };

        // Run prune
        let mut can_stay = vec![true; component.n_variables()];
        prune(
            &mut create_var_to_constraint(&component),
            0,
            &mut can_stay,
            &mut component,
        );

        let expected = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A { a_to_bc(a) -> [b, c]; }
        };

        assert_eq!(component, expected);
        assert_eq!(can_stay, vec![false, false, false]);
    }

    #[test]
    fn prune_removes_writes_before_selecting() {
        let mut component = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A {
                a_to_bc(a) -> [b, c];
                ab_to_ac(a, b) -> [a, c];
                ac_to_ab(a, c) -> [a, b];
            }
        };

        // Run prune
        let mut can_stay = vec![true; component.n_variables()];
        prune(
            &mut create_var_to_constraint(&component),
            0,
            &mut can_stay,
            &mut component,
        );

        let expected = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A { a_to_bc(a) -> [b, c]; }
        };

        assert_eq!(component, expected);
        assert_eq!(can_stay, vec![false, false, false]);
    }

    #[test]
    fn prune_keeps_ambiguous() {
        let mut component = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A {
                ab_to_c(a, b) -> [c];
                ac_to_b(a, c) -> [b];
                bc_to_a(b, c) -> [a];
            }
        };

        // Run prune
        let mut can_stay = vec![true; component.n_variables()];
        prune(
            &mut create_var_to_constraint(&component),
            0,
            &mut can_stay,
            &mut component,
        );

        let expected = dummy_component! {
            let a, b, c;
            constraint StayA {
                id(a) -> [a];
            }
            constraint A {
                ab_to_c(a, b) -> [c];
                ac_to_b(a, c) -> [b];
            }
        };

        assert_eq!(component, expected);
        assert_eq!(can_stay, vec![false, true, true]);
    }

    #[test]
    fn prune_dense() {
        let mut component = dummy_component! {
            let a, b, c, d, e, f, g;
            constraint StayA {
                id(a) -> [a];
            }
            constraint Ab {
                ab1(a) -> [b];
                ab2(b) -> [a];
            }
            constraint Ac {
                ac1(a) -> [c];
                ac2(c) -> [a];
            }
            constraint Ad {
                ad1(a) -> [d];
                ad2(d) -> [a];
            }
            constraint De {
                de1(d) -> [e];
                de2(e) -> [d];
            }
            constraint Df {
                df1(d) -> [f];
                df2(f) -> [d];
            }
            constraint Dg {
                dg1(d) -> [g];
                dg2(g) -> [d];
            }
        };

        // Run prune
        let mut can_stay = vec![true; component.n_variables()];
        prune(
            &mut create_var_to_constraint(&component),
            0,
            &mut can_stay,
            &mut component,
        );

        let expected = dummy_component! {
            let a, b, c, d, e, f, g;
            constraint StayA {
                id(a) -> [a];
            }
            constraint Ab {
                ab1(a) -> [b];
            }
            constraint Ac {
                ac1(a) -> [c];
            }
            constraint Ad {
                ad1(a) -> [d];
            }
            constraint De {
                de1(d) -> [e];
            }
            constraint Df {
                df1(d) -> [f];
            }
            constraint Dg {
                dg1(d) -> [g];
            }
        };

        assert_eq!(component, expected);
        assert_eq!(can_stay, vec![false; expected.n_variables()]);
    }

    #[test]
    fn ladder() {
        let mut component = dummy_component! {
            let a, b, c, d;
            constraint StayA {
                id(a) -> [a];
            }
            constraint StayB {
                id(b) -> [b];
            }
            constraint UpperLeft {
                abc(a, b) -> [c];
                acb(a, c) -> [b];
            }
            constraint LowerRight {
                bcd(b, c) -> [d];
                bdc(b, d) -> [c];
                cdb(c, d) -> [b];
            }
        };
        let mut can_stay = vec![true; component.n_variables()];
        can_stay[0] = false;
        prune(
            &mut create_var_to_constraint(&component),
            1,
            &mut can_stay,
            &mut component,
        );
        let expected = dummy_component! {
            let a, b, c, d;
            constraint StayA {
                id(a) -> [a];
            }
            constraint StayB {
                id(b) -> [b];
            }
            constraint UpperLeft {
                abc(a, b) -> [c];
            }
            constraint LowerRight {
                bcd(b, c) -> [d];
            }
        };
        pretty_assertions::assert_eq!(component, expected);
    }
}
