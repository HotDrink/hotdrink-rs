//! Types for constraint satisfaction methods.
//! A [`Method`] in a [`Constraint`](super::Constraint) should enforce the relation
//! that the constraint represents.

use super::{
    generation_id::GenerationId,
    solve_error::{Reason, SolveError},
};
use crate::{
    algorithms::Vertex,
    event::{Event, EventWithLocation},
    model::{
        spec::{MethodFailure, MethodFunction, MethodResult, MethodSpec},
        variable_activation::{VariableActivation, VariableActivationInner},
    },
    thread::ThreadPool,
};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

/// A method for enforcing a [`Constraint`](super::Constraint).
/// It usually has a set of input-variables, a set of output-variables,
/// and a function for creating the outputs from the inputs.
pub struct Method<T> {
    is_stay: bool,
    name: String,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    apply: MethodFunction<T>,
}

impl<T> Clone for Method<T> {
    fn clone(&self) -> Self {
        Self {
            is_stay: self.is_stay,
            name: self.name.clone(),
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            apply: self.apply.clone(),
        }
    }
}

impl<T> PartialEq for Method<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_stay == other.is_stay
            && self.name == other.name
            && self.inputs == other.inputs
            && self.outputs == other.outputs
    }
}

impl<T> Eq for Method<T> {}

impl<T> Debug for Method<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({:?} -> {:?})", self.name, self.inputs, self.outputs)
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
            is_stay: false,
            name,
            inputs,
            outputs,
            apply,
        }
    }

    /// Apply the inner function of this method
    fn apply(&self, input: Vec<Arc<T>>) -> MethodResult<Arc<T>> {
        // Verify that all inputs are defined
        if input.len() != self.n_inputs() {
            return Err(MethodFailure::WrongInputCount(self.n_inputs(), input.len()));
        }
        // Compute output
        let output = (self.apply)(input)?;
        // Verify that all outputs are defined
        if output.len() != self.n_outputs() {
            return Err(MethodFailure::WrongOutputCount(
                self.n_outputs(),
                output.len(),
            ));
        }

        Ok(output)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// A [`VariableActivationInner`] wrapped in an [`Arc`] and a [`Mutex`] so that it can be shared.
pub type SharedVariableActivationInner<T> = Arc<Mutex<VariableActivationInner<T, SolveError>>>;

fn handle_error<T>(
    output_indices: &[usize],
    shared_states: &Arc<Vec<SharedVariableActivationInner<T>>>,
    general_callback: &(impl Fn(EventWithLocation<'_, T, SolveError>) + Send + 'static),
    generation: GenerationId,
    errors: Vec<SolveError>,
) {
    log::error!("{:?}", errors);
    for &o in output_indices {
        general_callback(EventWithLocation::new(o, generation, Event::Error(&errors)));
    }
    for shared_state in shared_states.iter() {
        shared_state.lock().unwrap().set_error(errors.clone());
    }
}

impl<T> Method<T> {
    /// Calls the method with the provided arguments, but spawns off the computation in a different thread.
    /// Instead of waiting for the values to arrive, return a list of `Value`s that will eventually resolve to them.
    pub(crate) fn activate(
        &self,
        inputs: Vec<impl Into<VariableActivation<T, SolveError>>>,
        shared_states: Vec<SharedVariableActivationInner<T>>,
        location: (String, String),
        generation: GenerationId,
        pool: &mut impl ThreadPool,
        general_callback: impl Fn(EventWithLocation<'_, T, SolveError>) + Send + 'static,
    ) -> Vec<VariableActivation<T, SolveError>>
    where
        T: Send + Sync + 'static + Debug,
        Method<T>: Vertex,
    {
        // Convert the input to `Value`s that we can await.
        // Using `Into<Value<T>>` for convenience, so that
        // one can pass in a vector of non-futures too.
        let inputs: Vec<VariableActivation<T, SolveError>> =
            inputs.into_iter().map(|v| v.into()).collect();
        let n_inputs = self.n_inputs();
        let n_outputs = self.n_outputs();
        let output_indices = self.outputs().to_vec();
        let m_name = self.name().to_string();
        let (component, constraint) = location;

        log::trace!("Activating {}", &m_name);

        let shared_states = Arc::new(shared_states);
        let shared_states_clone = shared_states.clone();

        // Set pending
        for &o in &output_indices {
            general_callback(EventWithLocation::new(o, generation, Event::Pending));
        }

        // We need a clone of the computation to move into the thread
        let f = self.apply.clone();

        // Run the computation in another thread, which
        // will eventually put the computed values in
        // the shared_state slots.
        let handle = pool
            .execute(move || {
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
                        Err(es) => errors.extend(es),
                    }
                }

                // If any errors, propagate and abort.
                if !errors.is_empty() {
                    handle_error(
                        &output_indices,
                        &shared_states_clone,
                        &general_callback,
                        generation,
                        errors,
                    );
                    return;
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
                log::trace!("{}({}) = {:?}", m_name, formatted_inputs, result);

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
                                Event::Ready(&res),
                            ));
                            let mut shared_state = st.lock().unwrap();
                            // Set the new value
                            shared_state.set_value_arc(res);
                            // Notify the async runtime that the value is ready
                            if let Some(waker) = shared_state.waker_mut().take() {
                                waker.wake();
                            }
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
        let values = shared_states
            .iter()
            .map(|st| VariableActivation {
                inner: st.clone(),
                producer: Some(handle.clone()),
            })
            .collect();

        values
    }
}

impl<T> Vertex for Method<T> {
    /// Get the indices of the inputs to this method
    fn inputs(&self) -> &[usize] {
        &self.inputs
    }
    /// Get the indices of the outputs to this method
    fn outputs(&self) -> &[usize] {
        &self.outputs
    }

    fn stay(index: usize) -> Self {
        Self {
            is_stay: true,
            name: format!("_stay_{}", index),
            inputs: vec![index],
            outputs: vec![index],
            apply: Arc::new(|_| panic!("stay constraints should not be run")),
        }
    }

    fn is_stay(&self) -> bool {
        self.is_stay
    }
}
