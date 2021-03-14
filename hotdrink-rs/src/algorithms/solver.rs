//! Functions for executing plans created by a planner.
//! If provided with a plan and the current values of a component, they will
//! ensure that the all constraints are enforced.

use super::hierarchical_planner::{OwnedEnforcedConstraint, Vertex};
use crate::{
    data::{
        method::Method,
        solve_error::SolveError,
        traits::{MethodFailure, MethodLike, PlanError},
        variable_activation::{SharedState, VariableActivation},
    },
    event::GeneralEvent,
    thread::thread_pool::ThreadPool,
};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

/// Given a plan and a set of current values, execute the
/// methods one by one while updating the current values
/// until all of them are done.
pub fn solve<T: Clone, M>(plan: &[&M], old_values: &mut Vec<T>) -> Result<(), MethodFailure>
where
    M: MethodLike<Arg = T>,
{
    // Store in separate variable temporarily to fail atomically
    let mut current_values = old_values.clone();

    for m in plan {
        // Pick inputs from current values
        let inputs = m
            .inputs()
            .iter()
            .map(|&i| current_values[i].clone())
            .collect();
        // Compute outputs
        let outputs = m.apply(inputs)?;
        // Re-insert outputs
        outputs
            .into_iter()
            .zip(m.outputs())
            .for_each(|(v, &o)| current_values[o] = v);
    }

    // All method calls were ok, we can now update the mutable reference
    *old_values = current_values;

    Ok(())
}

/// Solves a component by executing a plan concurrently.
///
/// The following arguments are required:
/// 1. A plan to execute.
/// 2. The current values of a component.
/// 3. The component name for better error messages.
/// 4. The generation to know which solve new values came from.
/// 5. A thread pool implementation for running methods in a plan.
/// 6. A callback to pass new produced values to. These events include the component name and the generation.
pub fn par_solve<T>(
    plan: &[OwnedEnforcedConstraint<Method<T>>],
    current_values: &mut Vec<VariableActivation<T, SolveError>>,
    component_name: String,
    generation: usize,
    pool: &mut impl ThreadPool,
    general_callback: impl Fn(GeneralEvent<T, SolveError>) + Send + 'static + Clone,
) -> Result<(), PlanError>
where
    T: Clone + Send + 'static + Debug,
{
    if !plan.is_empty() {
        log::debug!("Solving {} with plan {:?}", component_name, plan);
    }

    for osc in plan {
        let constraint_name = osc.name();
        let m = osc.method();

        // Pick inputs from current values
        let inputs = m
            .inputs()
            .iter()
            .map(|&i| current_values[i].clone())
            .collect();

        let mut shared_states = Vec::with_capacity(m.outputs().len());
        for &o in m.outputs() {
            let previous_activation = &mut current_values[o];
            // Cancel old activation
            previous_activation.cancel();
            // Keep the old value from the previous state, but set to pending
            let shared_state = SharedState::from_previous(previous_activation.shared_state());
            shared_states.push(Arc::new(Mutex::new(shared_state)));
        }

        // Compute outputs
        let outputs = m.activate(
            inputs,
            shared_states,
            (component_name.to_owned(), constraint_name.to_owned()),
            generation,
            pool,
            general_callback.clone(),
        );

        // Re-insert outputs
        outputs
            .into_iter()
            .zip(m.outputs())
            .for_each(|(v, &o)| current_values[o] = v);
    }

    Ok(())
}
