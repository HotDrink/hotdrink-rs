use super::component::{Component, Constraint, Method};
use crate::{
    algorithms::simple_planner::VariableRefCounter, ComponentLike, ConstraintLike, MethodLike,
};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;

pub struct MustsAndStays<'a, 'b, T>(&'a [T], &'b [T]);

impl<'a, 'b, T> MustsAndStays<'a, 'b, T> {
    pub fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }
}

impl<'a, 'b, T> std::ops::Index<usize> for MustsAndStays<'a, 'b, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.0.len() {
            &self.0[index]
        } else {
            &self.1[index - self.0.len()]
        }
    }
}

pub fn simple_planner<'a>(
    n_variables: usize,
    constraints: &'a MustsAndStays<'a, 'a, Constraint>,
) -> Option<Vec<&'a Method>> {
    let mut plan = Vec::with_capacity(constraints.len());
    let mut remaining_constraints: HashSet<usize> = (0..constraints.len()).collect();

    // Find the total use-count for each variable (n where n number of constraints)
    let mut variables = vec![VariableRefCounter::new(); n_variables];
    for ci in remaining_constraints.iter() {
        let constraint = &constraints[*ci];
        // Count the constraints that reference each variable
        for var_id in constraint.references() {
            let variable = &mut variables[var_id];
            variable.add_reference(*ci);
        }
    }

    // Add initial interesting variables (used once, only as output in a single constraint)
    let mut potentially_free_variables = VecDeque::with_capacity(n_variables);
    for (idx, variable) in variables.iter_mut().enumerate() {
        if variable.is_free() {
            potentially_free_variables.push_back(idx);
        }
    }

    // Start removal loop
    while !remaining_constraints.is_empty() {
        // Pick the first current interesting variable
        let idx = potentially_free_variables.pop_front()?;
        let interesting_variable = variables[idx].clone();

        // May have become uninteresting since we added it.
        // If it were interesting at some point, but is no more, then
        // the number of references must be 0.
        if !interesting_variable.is_free() {
            continue;
        }

        // Get the constraints with this variable, should only be one
        for ci in interesting_variable.referencing_constraints() {
            let constraint = &constraints[*ci];
            // Find all methods that write to free variables
            let free_methods = constraint
                .wrapped_methods()
                .iter()
                .filter(|m| m.outputs().iter().all(|&o| variables[o].is_free()));

            // Sort them by how many outputs they have
            let mut sorted_free_methods =
                free_methods.sorted_by(|m1, m2| m1.outputs().len().cmp(&m2.outputs().len()));
            // Get the first one if one exists
            let first_free_method = sorted_free_methods.next();
            if let Some(m) = first_free_method {
                // Add this method to the plan
                plan.push(m);
                remaining_constraints.remove(ci);

                // Remove all references to this constraint
                for vi in constraint.references() {
                    let variable = &mut variables[vi];
                    variable.remove_reference(*ci);
                    if variable.is_free() {
                        potentially_free_variables.push_back(vi);
                    }
                }
            }
        }
    }

    Some(plan)
}

pub fn hierarchical_planner<'a, T>(
    component: &'a Component<T>,
    ranking: &[usize],
) -> Option<Vec<&'a Method>> {
    let musts = component.constraints();
    let mut stays: Vec<Constraint> = Vec::new();
    let n_variables = component.n_variables();

    let mut plan = None;

    for var_id in ranking {
        // Add stay constraint
        let mut stay_constraint = Constraint::new(vec![*var_id]);
        stay_constraint
            .add_method(Method::new(&[*var_id], &[*var_id]).unwrap())
            .unwrap();
        stays.push(stay_constraint);

        let musts_and_stays = MustsAndStays(musts, &stays);

        // Try with this stay constraint
        match simple_planner(n_variables, &musts_and_stays) {
            Some(new_plan_indices) => {
                plan = Some(new_plan_indices);
            }
            None => {
                // Remove stay again
                stays.pop();
            }
        }
    }

    // Extract methods by indices
    plan.or_else(|| simple_planner(n_variables, MustsAndStays(musts, &[])))
}

/// Counts the neighboring constraints of a constraint.
#[derive(Clone)]
struct ConstraintRefCounter {
    /// All other constraints that use variables of this constraint.
    references: Vec<usize>,
    /// Which other constraints each method writes to.
    method_writes: Vec<Vec<usize>>,
    /// The lowest number of other constraints a method writes to.
    least: usize,
}

impl ConstraintRefCounter {
    fn new() -> Self {
        Self {
            references: Vec::new(),
            method_writes: Vec::new(),
            least: std::usize::MAX,
        }
    }
    fn count_constraint_refs<Comp: ComponentLike>(component: &Comp) -> Vec<Self> {
        let variables = VariableRefCounter::count_variable_refs(component);

        // Use the map to generate neighbors
        let mut constraint_ref_counters =
            vec![ConstraintRefCounter::new(); component.constraints().len()];

        for (self_ci, constraint) in component.constraints().iter().enumerate() {
            // Add total references
            for &vi in constraint.variables() {
                let refs = variables[vi].referencing_constraints();
                for &ci in refs {
                    if ci != self_ci {
                        constraint_ref_counters[self_ci].add_reference(ci);
                    }
                }
            }
            // Add method writes
            for (mi, m) in constraint.methods().iter().enumerate() {
                constraint_ref_counters[self_ci]
                    .method_writes
                    .push(Vec::new());
                for &o in m.outputs() {
                    let refs = variables[o].referencing_constraints();
                    for &ci in refs {
                        if ci != self_ci {
                            constraint_ref_counters[self_ci].add_method_write(mi, ci);
                        }
                    }
                }
            }
        }

        // Sort and remove duplicates, and update least
        for crc in &mut constraint_ref_counters {
            crc.references.sort_unstable();
            crc.references.dedup();
            for writes in &mut crc.method_writes {
                writes.sort_unstable();
                writes.dedup();

                if writes.len() < crc.least {
                    crc.least = writes.len();
                }
            }
        }

        constraint_ref_counters
    }
    fn has_free_method(&self) -> bool {
        self.least == 0
    }
    pub fn add_reference(&mut self, index: usize) {
        self.references.push(index);
    }
    pub fn add_method_write(&mut self, mi: usize, ci: usize) {
        self.method_writes[mi].push(ci);
    }
    pub fn remove_reference(&mut self, index: usize) {
        // Remove total reference
        self.references.retain(|ci| ci != &index);
        // Remove from methods, and update least
        for v in &mut self.method_writes {
            v.retain(|ci| ci != &index);
            let new_len = v.len();
            if new_len < self.least {
                self.least = new_len;
            }
        }
    }
    pub fn references(&self) -> &[usize] {
        &self.references
    }
    pub fn method_writes(&self) -> &[Vec<usize>] {
        &self.method_writes
    }
}

#[allow(clippy::needless_lifetimes)]
pub fn new_new_simple_planner<'a, M, C, Comp>(component: &'a Comp) -> Option<Vec<&'a M>>
where
    M: MethodLike,
    C: ConstraintLike<Method = M> + 'a + Debug,
    Comp: ComponentLike<Constraint = C>,
{
    let mut plan = Vec::with_capacity(component.constraints().len());
    let constraints = component.constraints();
    let mut remaining_constraints = constraints.len();
    let mut constraint_ref_counters = ConstraintRefCounter::count_constraint_refs(component);

    let mut interesting_constraints = VecDeque::new();
    for (ci, crc) in constraint_ref_counters.iter().enumerate() {
        if crc.has_free_method() {
            interesting_constraints.push_back(ci);
        }
    }

    while remaining_constraints != 0 {
        let ci = interesting_constraints.pop_front()?;
        let constraint = &constraints[ci];
        let crc = &constraint_ref_counters[ci];
        if !crc.has_free_method() {
            continue;
        }

        // Find free method
        let mut least = None;
        let mut least_m = None;
        for (mi, mw) in crc.method_writes().iter().enumerate() {
            // Not free
            if !mw.is_empty() {
                continue;
            }
            // Select method with fewest outputs
            let m = &constraint.methods()[mi];
            if least.is_none() || Some(m.outputs().len()) < least {
                least = Some(m.outputs().len());
                least_m = Some(m);
            }
        }

        // If free method found, add to plan and remove constraint
        if let Some(m) = least_m {
            plan.push(m);
            remaining_constraints -= 1;

            let references = crc.references().to_owned();
            for other in references {
                constraint_ref_counters[other].remove_reference(other);
            }
        }
    }

    Some(plan)
}
