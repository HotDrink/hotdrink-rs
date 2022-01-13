//! Types for constraint satisfaction methods.
//! A [`Method`] in a [`Constraint`](super::Constraint) should enforce the relation
//! that the constraint represents.

use super::generation_id::GenerationId;
use crate::{
    event::{Event, EventWithLocation, Ready},
    executor::MethodExecutor,
    model::activation::{Activation, ActivationInner},
    planner::{MethodFailure, MethodFunction, MethodResult, MethodSpec, Vertex},
    solver::{Reason, SolveError},
};
use core::slice;
use derivative::Derivative;
use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

/// The inner representation of a [`Method`].
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug(bound = ""), PartialEq(bound = ""), Eq)]
pub enum MethodInner<T> {
    /// A stay method.
    Stay(usize),
    /// A normal method.
    Normal {
        name: String,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        #[derivative(Debug = "ignore", PartialEq = "ignore")]
        apply: MethodFunction<T>,
    },
}

/// A method for enforcing a [`Constraint`](super::Constraint).
/// It usually has a set of input-variables, a set of output-variables,
/// and a function for creating the outputs from the inputs.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), PartialEq(bound = ""), Eq)]
pub struct Method<T> {
    inner: MethodInner<T>,
}

impl<T> Debug for Method<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}({:?} -> {:?})",
            self.name(),
            self.inputs(),
            self.outputs()
        )
    }
}

impl<T> MethodSpec for Method<T> {
    type Arg = T;

    fn new(
        name: String,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        apply: MethodFunction<T>,
    ) -> Self {
        Self {
            inner: MethodInner::Normal {
                name,
                inputs,
                outputs,
                apply,
            },
        }
    }

    /// Apply the inner function of this method
    fn apply(&self, input: Vec<Arc<T>>) -> MethodResult<Arc<T>> {
        // Verify that all inputs are defined
        if input.len() != self.n_inputs() {
            return Err(MethodFailure::WrongInputCount(self.n_inputs(), input.len()));
        }
        // Compute output
        let output = match &self.inner {
            MethodInner::Stay(_) => input,
            MethodInner::Normal { apply, .. } => apply(input)?,
        };
        // Verify that all outputs are defined
        if output.len() != self.n_outputs() {
            return Err(MethodFailure::WrongOutputCount(
                self.n_outputs(),
                output.len(),
            ));
        }

        Ok(output)
    }

    fn name(&self) -> Option<&str> {
        match &self.inner {
            MethodInner::Stay(_) => None,
            MethodInner::Normal { name, .. } => Some(name),
        }
    }
}

fn handle_error<T>(
    output_indices: &[usize],
    shared_states: &[Arc<RwLock<ActivationInner<T>>>],
    general_callback: &(impl Fn(EventWithLocation<'_, T, SolveError>) + Send + 'static),
    generation: GenerationId,
    errors: Vec<SolveError>,
) {
    for &o in output_indices {
        general_callback(EventWithLocation::new(o, generation, Event::Error(&errors)));
    }
    for shared_state in shared_states.iter() {
        shared_state.write().unwrap().set_error(errors.clone());
    }
}

impl<T> Method<T> {
    /// Calls the method with the provided arguments, but spawns off the computation in a different thread.
    /// Instead of waiting for the values to arrive, return a list of `Value`s that will eventually resolve to them.
    pub(crate) fn activate(
        &self,
        inputs: Vec<impl Into<Activation<T>>>,
        shared_states: Vec<Arc<RwLock<ActivationInner<T>>>>,
        location: (String, String),
        generation: GenerationId,
        me: &impl MethodExecutor,
        general_callback: impl Fn(EventWithLocation<'_, T, SolveError>) + Send + 'static,
    ) -> Vec<Activation<T>>
    where
        T: Send + Sync + 'static + Debug,
        Method<T>: Vertex,
    {
        // Convert the input to `Value`s that we can await.
        // Using `Into<Value<T>>` for convenience, so that
        // one can pass in a vector of non-futures too.
        let inputs: Vec<Activation<T>> = inputs.into_iter().map(|v| v.into()).collect();
        let n_inputs = self.n_inputs();
        let n_outputs = self.n_outputs();
        let output_indices = self.outputs().to_vec();
        let m_name = self.name().unwrap_or("None").to_string();
        let (component, constraint) = location;
        let component_clone = component.clone();
        let constraint_clone = constraint.clone();
        let method_clone = m_name.clone();

        log::trace!("Activating {}", &m_name);

        let shared_states = shared_states;
        let shared_states_clone = shared_states.clone();

        // We need a clone of the computation to move into the thread
        let f = match &self.inner {
            MethodInner::Stay(_) => Arc::new(Ok),
            MethodInner::Normal { apply, .. } => apply.clone(),
        };

        // Run the computation in another thread, which
        // will eventually put the computed values in
        // the shared_state slots.
        let handle = me
            .schedule(move || {
                // Block on all the futures. This is ok
                // since we are not on the main thread.
                let joined_inputs = futures::future::join_all(inputs);
                let input_results = futures::executor::block_on(joined_inputs);
                let formatted_inputs = format!("{:?}", &input_results);

                // Split ok and erroneous inputs
                let mut inputs = Vec::new();
                let mut errors = Vec::new();
                for state in input_results {
                    match state {
                        Ok(value) => inputs.push(value),
                        Err((value, error_data)) => {
                            inputs.push(value);
                            errors.extend(error_data.errors().clone());
                        }
                    }
                }

                // If any errors in input, propagate to outputs
                if !errors.is_empty() {
                    handle_error(
                        &output_indices,
                        &shared_states_clone,
                        &general_callback,
                        generation,
                        errors,
                    );
                }

                // Check that all inputs are provided
                if inputs.len() != n_inputs {
                    let error = SolveError::new(
                        component.to_owned(),
                        constraint.to_owned(),
                        m_name.clone(),
                        Reason::MethodFailure(MethodFailure::WrongInputCount(
                            n_inputs,
                            inputs.len(),
                        )),
                    );
                    handle_error(
                        &output_indices,
                        &shared_states_clone,
                        &general_callback,
                        generation,
                        vec![error],
                    );
                    return;
                }

                // Compute the result
                let result = f(inputs);
                log::info!("{}({}) = {:?}", m_name, formatted_inputs, result);

                // Inspect it
                match result {
                    // The method call was successful, zip the values into the shared states
                    Ok(outputs) => {
                        // Undefined output variables
                        if outputs.len() != n_outputs {
                            let error = SolveError::new(
                                component.to_owned(),
                                constraint.to_owned(),
                                m_name.clone(),
                                Reason::MethodFailure(MethodFailure::WrongOutputCount(
                                    n_outputs,
                                    outputs.len(),
                                )),
                            );
                            handle_error(
                                &output_indices,
                                &shared_states_clone,
                                &general_callback,
                                generation,
                                vec![error],
                            );
                            return;
                        }
                        // Place values in slots, and send ready events
                        for ((st, res), &o) in
                            shared_states_clone.iter().zip(outputs).zip(&output_indices)
                        {
                            general_callback(EventWithLocation::new(
                                o,
                                generation,
                                Event::Ready(Ready::Changed(&res)),
                            ));
                            let mut shared_state = st.write().unwrap();
                            // Set the new value
                            shared_state.set_value_arc(res);
                        }
                    }
                    // The method call failed
                    Err(e) => {
                        let error = SolveError::new(
                            component.to_owned(),
                            constraint.to_owned(),
                            m_name.clone(),
                            Reason::MethodFailure(e),
                        );
                        handle_error(
                            &output_indices,
                            &shared_states_clone,
                            &general_callback,
                            generation,
                            vec![error],
                        );
                    }
                }
            })
            .expect("Could not spawn worker");

        // Wrap the shared states and the thread references in `Value`s
        let output: Vec<Activation<T>> = shared_states
            .iter()
            .map(|st| Activation {
                inner: st.clone(),
                producer: Some(handle.clone()),
            })
            .collect();

        // Set activation values to canceled when their values are no longer needed
        let mut output_clone: Vec<Activation<T>> = output.iter().map(|a| a.weak_clone()).collect();
        handle.on_drop(move || {
            for ss in &mut output_clone {
                ss.cancel(SolveError::new(
                    component_clone.clone(),
                    constraint_clone.clone(),
                    method_clone.clone(),
                    Reason::Cancelled,
                ));
            }
        });

        output
    }
}

impl<T> Vertex for Method<T> {
    /// Get the indices of the inputs to this method
    fn inputs(&self) -> &[usize] {
        match &self.inner {
            MethodInner::Stay(index) => slice::from_ref(index),
            MethodInner::Normal { inputs, .. } => inputs,
        }
    }
    /// Get the indices of the outputs to this method
    fn outputs(&self) -> &[usize] {
        match &self.inner {
            MethodInner::Stay(index) => slice::from_ref(index),
            MethodInner::Normal { outputs, .. } => outputs,
        }
    }

    fn stay(index: usize) -> Self {
        Self {
            inner: MethodInner::Stay(index),
        }
    }

    fn is_stay(&self) -> bool {
        matches!(self.inner, MethodInner::Stay(_))
    }
}
