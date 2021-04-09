use super::component::{Component, Constraint, Method};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct VarIndex(usize);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct MethodIndex(usize);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ConstraintIndex(usize);

/// A mutable wrapper around a `Component`.
/// The internal `Component` is read-only, but the wrapper
/// works on indices that can change.
pub trait ConstraintGraphT<'a, T: 'a>: From<&'a Component<T>> {
    /// Get constraints connected to the specified variable.
    fn get_constraints(&self, index: VarIndex) -> &[ConstraintIndex];
    /// Get a specific constraint.
    fn get_constraint(&self, index: ConstraintIndex) -> &Constraint;
    /// Add a new constraint (usually a stay constraint).
    fn add_constraint(&mut self, constraint: Constraint) -> ConstraintIndex;
    /// Remove a given constraint, and remove references to it.
    fn remove_constraint(&mut self, index: ConstraintIndex);

    /// Notes which method is used to satisfy this constraint, and removes references to it.
    fn enforce(&mut self, ci: ConstraintIndex, mi: MethodIndex);

    /// Get free methods of this constraint.
    fn free_methods(&mut self, index: ConstraintIndex) -> Vec<&'a Method>;

    /// Reset which constraints have been enforced already.
    fn remaining_constraints(&self) -> &[ConstraintIndex];
    /// Returns the first remaining free variable.
    /// Skips any that have become unused since being found.
    fn pop_free_variable(&mut self) -> Option<VarIndex>;

    /// Get variables connected to the specified constraint.
    fn get_variables(&self, index: ConstraintIndex) -> &[usize];

    /// Check if a given variable is free.
    fn is_free(&self, index: VarIndex) -> bool;

    /// Prune the constraint graph from the specified variable.
    fn prune(&mut self, var_idx: VarIndex);

    /// If each constraint has a single method left, return the plan.
    fn collect_plan(&self) -> Option<Vec<&'a Method>>;

    /// If each constraint has a single method left, return the plan.
    /// Also filter out any stay methods that were added.
    /// Topologically sort the result.
    fn collect_plan_sorted(&self) -> Option<Vec<&'a Method>>;
}

/// A simple planner that is abstracted away behind the `ConstraintGraphT` trait.
pub fn simple_planner<'a, T: 'a, CG>(mut cg: CG) -> Option<Vec<&'a Method>>
where
    CG: ConstraintGraphT<'a, T>,
{
    // While there are constraints left
    while !cg.remaining_constraints().is_empty() {
        // Find a free variable
        let index = cg.pop_free_variable()?;

        // Look at constraints referencing this variable
        if let [ci] = cg.get_constraints(index) {
            // Find a free method
            let mut free_methods = cg.free_methods(*ci);
            // Sort by number of outputs
            free_methods.sort_by_key(|m| m.outputs().len());
            // Get the first one
            if let Some(free_method) = free_methods.get(0) {}
        } else {
            panic!("More constraints were interested in this variable!");
        }
    }

    cg.collect_plan()
}

pub fn hierarchical_planner<'a, T: 'a, CG>(cg: CG) -> Option<Vec<&'a Method>>
where
    CG: ConstraintGraphT<'a, T>,
{
    None
}
