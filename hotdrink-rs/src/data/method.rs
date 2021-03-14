use super::solve_error::{Reason, SolveError};
use crate::data::variable_activation::State;
use crate::{
    algorithms::hierarchical_planner::Vertex,
    data::{
        traits::{MethodFailure, MethodFunction, MethodLike, MethodResult},
        variable_activation::{SharedState, VariableActivation},
    },
    event::{Event, GeneralEvent},
    thread::thread_pool::ThreadPool,
};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

/// A method for enforcing a [`Constraint`](crate::Constraint).
/// It usually has a set of input-variables, a set of output-variables,
/// and a function for creating the outputs from the inputs.
#[derive(Clone)]
pub enum Method<T> {
    /// A stay-method that reads a value from a variable, then writes the same one back.
    /// This works like an identity function.
    Stay(usize),
    /// Any other method.
    Normal(String, Vec<usize>, Vec<usize>, MethodFunction<T>),
}

impl<T> PartialEq for Method<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Method::Stay(var1), Method::Stay(var2)) => var1 == var2,
            (Method::Normal(_, is1, os1, _), Method::Normal(_, is2, os2, _)) => {
                is1 == is2 && os1 == os2
            }
            _ => false,
        }
    }
}

impl<T> Eq for Method<T> {}

impl<T> Debug for Method<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Stay(var) => write!(f, "Method::Stay({})", var),
            Method::Normal(name, inputs, outputs, _) => {
                write!(f, "{}({:?} -> {:?})", name, inputs, outputs)
            }
        }
    }
}

impl<T> MethodLike for Method<T> {
    type Arg = T;

    fn new(
        name: String,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        apply: MethodFunction<T>,
    ) -> Self {
        Method::Normal(name, inputs, outputs, apply)
    }

    /// Apply the inner function of this method
    fn apply(&self, input: Vec<T>) -> MethodResult<T> {
        // Verify that all inputs are defined
        if input.len() != self.n_inputs() {
            return Err(MethodFailure::WrongInputCount(self.n_inputs(), input.len()));
        }
        match self {
            Method::Stay(_) => Ok(vec![]),
            Method::Normal(_, _, _, apply) => {
                // Perform computation
                let output = (apply)(input)?;
                // Verify that all outputs are defined
                if output.len() != self.n_outputs() {
                    return Err(MethodFailure::WrongOutputCount(
                        self.n_outputs(),
                        output.len(),
                    ));
                }
                // Return the result
                Ok(output)
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Method::Stay(_) => "<stay>",
            Method::Normal(name, _, _, _) => &name,
        }
    }
}

/// A [`SharedState`] wrapped in an [`Arc`] and a [`Mutex`] so that it can be shared.
pub type SharedSharedState<T> = Arc<Mutex<SharedState<T, SolveError>>>;

fn handle_error<T>(
    output_indices: &[usize],
    shared_states: &Arc<Vec<SharedSharedState<T>>>,
    general_callback: &(impl Fn(GeneralEvent<T, SolveError>) + Send + 'static),
    generation: usize,
    errors: Vec<SolveError>,
) where
    T: Clone,
{
    for &o in output_indices {
        general_callback(GeneralEvent::new(
            o,
            generation,
            Event::Error(errors.clone()),
        ));
    }
    for shared_state in shared_states.iter() {
        shared_state.lock().unwrap().set_error(errors.clone());
    }
}

impl<T> Method<T> {
    /// Calls the method with the provided arguments, but spawns off the computation in a different thread.
    /// Instead of waiting for the values to arrive, return a list of `Value`s that will eventually resolve to them.
    pub fn activate(
        &self,
        inputs: Vec<impl Into<VariableActivation<T, SolveError>>>,
        shared_states: Vec<SharedSharedState<T>>,
        location: (String, String),
        generation: usize,
        pool: &mut impl ThreadPool,
        general_callback: impl Fn(GeneralEvent<T, SolveError>) + Send + 'static,
    ) -> Vec<VariableActivation<T, SolveError>>
    where
        T: Clone + Send + 'static + Debug,
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

        log::debug!("Activating {}", &m_name);

        let shared_states = Arc::new(shared_states);
        let shared_states_clone = shared_states.clone();

        // Set pending
        for &o in &output_indices {
            general_callback(GeneralEvent::new(o, generation, Event::Pending));
        }

        // We need a clone of the computation to move into the thread
        // TODO: Use `apply` instead to do some extra checks.
        let f: Arc<dyn Fn(Vec<T>) -> MethodResult<T> + Send + Sync + 'static> = match self {
            Method::Stay(_) => Arc::new(|_| Ok(vec![])),
            Method::Normal(_, _, _, fun) => fun.clone(),
        };

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
                for (value, state) in input_results {
                    match state {
                        State::Pending => {
                            panic!("How did this happen? Await should have waited longer")
                        }
                        State::Ready => inputs.push(value),
                        State::Error(es) => errors.extend(es),
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
                log::debug!("{}({}) = {:?}", m_name, formatted_inputs, result);

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
                            general_callback(GeneralEvent::new(
                                o,
                                generation,
                                Event::Ready(res.clone()),
                            ));
                            let mut shared_state = st.lock().unwrap();
                            // Set the new value
                            shared_state.set_value(res);
                            // Notify the async runtime that the value is ready
                            if let Some(waker) = shared_state.waker.take() {
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
                shared_state: st.clone(),
                producer: Some(handle.clone()),
            })
            .collect();

        values
    }
}

impl<T> Vertex for Method<T> {
    /// Get the indices of the inputs to this method
    fn inputs(&self) -> &[usize] {
        match self {
            Method::Normal(_, inputs, _, _) => &inputs,
            Method::Stay(var) => std::slice::from_ref(&var),
        }
    }
    /// Get the indices of the outputs to this method
    fn outputs(&self) -> &[usize] {
        match self {
            Method::Normal(_, _, outputs, _) => &outputs,
            Method::Stay(var) => std::slice::from_ref(&var),
        }
    }

    fn stay(index: usize) -> Self {
        Method::Stay(index)
    }

    fn is_stay(&self) -> bool {
        matches!(self, Method::Stay(_))
    }
}

#[cfg(test)]
mod tests {
    // use std::sync::Arc;
    // use crate::{data::traits::MethodError, thread::pool::dummy_pool::DummyPool};
    // use super::Method;

    // #[test]
    // pub fn activate_test() {
    //     let m = Method::Normal(
    //         "m".to_string(),
    //         vec![0, 1],
    //         vec![2],
    //         Arc::new(|v: Vec<i32>| Ok(vec![v[0] + v[1]])),
    //     );
    //     // Verify good input gives good output
    //     let futures = m.activate(vec![3, 6], &mut DummyPool);
    //     match futures {
    //         Ok(futures) => futures::executor::block_on(async {
    //             let values = futures::future::join_all(futures).await;
    //             assert_eq!(values, vec![9]);
    //         }),
    //         Err(e) => panic!("This method call should not fail, but got {:?}", e),
    //     }

    //     // Verify that bad input gives error
    //     assert_eq!(
    //         m.activate(vec![3], &mut DummyPool),
    //         Err(MethodError::WrongInputCount)
    //     );
    // }
}
