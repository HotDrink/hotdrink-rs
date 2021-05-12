//! Functions for scheduling plans created by a planner.
//! If provided with a plan and the current values of a component,
//! [`schedule`] will schedule methods to be run in an appropriate order
//! using a method executor of choice.

use crate::{
    event::Event,
    event::{EventWithLocation, Ready},
    model::{
        activation::{Activation, ActivationInner, State},
        generation_id::GenerationId,
        variables::Variables,
    },
    thread::ThreadPool,
};
use crate::{
    model::Method,
    planner::{OwnedEnforcedConstraint, Vertex},
};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use super::SolveError;

/// Schedules methods of plan to be run on a method executor.
///
/// The following arguments are required:
/// 1. A plan to execute.
/// 2. The current values of a component.
/// 3. The component name for better error messages.
/// 4. The generation to know which solve new values came from.
/// 5. A thread pool implementation for running methods in a plan.
/// 6. A callback to pass new produced values to. These events include the component name and the generation.
pub(crate) fn schedule<T>(
    plan: &[OwnedEnforcedConstraint<Method<T>>],
    current_values: &mut Variables<Activation<T>>,
    component_name: String,
    generation: GenerationId,
    pool: &mut impl ThreadPool,
    general_callback: impl Fn(EventWithLocation<'_, T, SolveError>) + Send + 'static + Clone,
) where
    T: Send + Sync + 'static + Debug,
{
    if !plan.is_empty() {
        log::info!("Solving {}", component_name);
    }

    for osc in plan {
        let constraint_name = osc.name();
        let m = osc.method();
        log::info!("Activating {:?}", m);

        // Clear previous errors of inputs since the errors no longer apply
        for &o in m.inputs() {
            log::info!("Setting {} to ok", o);
            let activation = &current_values[o];
            let inner = activation.inner().lock().unwrap();
            if let State::Error(_) = inner.state() {
                general_callback(EventWithLocation::new(
                    o,
                    generation,
                    Event::Ready(Ready::Unchanged),
                ));
            }
        }

        // Set outputs to pending
        for &o in m.outputs() {
            log::info!("Setting {} to pending", o);
            general_callback(EventWithLocation::new(o, generation, Event::Pending));
        }

        // Pick inputs from current values
        let mut inputs: Vec<Activation<_>> = m
            .inputs()
            .iter()
            .map(|&i| current_values[i].clone())
            .collect();

        // Ignore errors from previous generation,
        // since they do no longer apply.
        // Errors of outputs will be overwritten anyway.
        for a in &mut inputs {
            a.clear_error();
        }

        let mut shared_states = Vec::with_capacity(m.outputs().len());
        for &o in m.outputs() {
            current_values.get_mut(o).unwrap().detach();
            // Keep the old value from the previous state, but set to pending
            let previous = current_values[o].clone();
            let shared_state = ActivationInner::new(previous, inputs.clone());
            shared_states.push(Arc::new(Mutex::new(shared_state)));
        }

        // Compute outputs
        let weak_clone_inputs = inputs.iter().map(|a| a.weak_clone()).collect();
        let outputs = m.activate(
            weak_clone_inputs,
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
            .for_each(|(v, &o)| current_values.set(o, v));
    }
}
