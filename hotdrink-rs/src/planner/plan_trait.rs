use super::EnforcedConstraint;
use super::{ComponentSpec, ConstraintSpec, MethodSpec, PlanError};
use std::fmt::Debug;

/// A trait for planners to implement.
pub trait Plan {
    /// Constructs a plan for the component.
    fn plan<T, M, C, Comp>(component: &Comp) -> Result<Vec<EnforcedConstraint<'_, T>>, PlanError>
    where
        M: MethodSpec<Arg = T> + Clone,
        C: ConstraintSpec<Method = M> + Debug + Clone,
        Comp: ComponentSpec<Constraint = C> + Clone;
}
