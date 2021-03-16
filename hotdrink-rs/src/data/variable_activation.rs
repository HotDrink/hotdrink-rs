//! Types for representing a [`VariableActivation`]. That is, the latest (possibly not computed yet) value of a variable.
//! Updating the values of a constraint system will not happen immediately, but the activations will be ready,
//! and act as futures or promises that eventually get the new value.

use crate::{event::Event, thread::thread_pool::TerminationHandle};
use futures::Future;
use std::{
    fmt::Debug,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

/// The possible states of a variable's value.
/// It starts off with being pending, and can
/// transition to `State::Ready` when its computation succeeds,
/// or `State::Error` if the computation fails.
#[derive(Clone, Debug, PartialEq)]
pub enum State<E> {
    /// The value is still being computed.
    Pending,
    /// The value was computed successfully.
    Ready,
    /// The computation of the value failed.
    Error(Vec<E>),
}

impl<E> Default for State<E> {
    fn default() -> Self {
        Self::Pending
    }
}

/// A callback to an [`Event`] sent from a call to [`ConstraintSystem::update`](crate::ConstraintSystem::update).
pub type EventCallback<T, E> = Arc<Mutex<dyn Fn(Event<T, E>) + Send>>;

/// Contains a slot for a value to be produced,
/// and one for a waker to be called when this happens.
pub struct SharedState<T, E> {
    value: T,
    state: State<E>,
    waker: Option<Waker>,
}

impl<T: Clone, E: Clone> SharedState<T, E> {
    /// Constructs a [`SharedState`] from another one.
    /// This is used as a way of having a fallback in case the new computation fails.
    /// That is, previous provides a default value.
    pub fn from_previous(previous: &Arc<Mutex<SharedState<T, E>>>) -> Self {
        // TODO: If previous is not done yet, it will take previous' old value.
        // We should wait for the previous to get a value,
        // and if that fails it will again use a reference to the previous one.
        let previous = previous.lock().unwrap();
        Self {
            value: previous.value.clone(),
            state: State::Pending,
            waker: None,
        }
    }

    /// Returns a reference to the current state.
    pub fn get_state(&self) -> &State<E> {
        &self.state
    }

    /// Returns a reference to the current value.
    pub fn current_value(&self) -> &T {
        &self.value
    }

    /// Set the state to [`State::Pending`].
    pub fn set_pending(&mut self) {
        self.state = State::Pending;
    }

    /// Sets the state to a successful value.
    pub fn set_value(&mut self, value: T) {
        self.value = value;
        self.state = State::Ready;
    }

    /// Set the state to a failed value.
    pub fn set_error(&mut self, errors: Vec<E>) {
        if let State::Error(previous_errors) = &mut self.state {
            previous_errors.extend(errors);
        } else {
            self.state = State::Error(errors)
        }
    }

    /// Returns a mutable reference to the [`Waker`].
    pub fn waker_mut(&mut self) -> &mut Option<Waker> {
        &mut self.waker
    }
}

impl<T, E> From<T> for SharedState<T, E> {
    fn from(value: T) -> Self {
        Self {
            value,
            state: State::Ready,
            waker: None,
        }
    }
}

impl<T: Debug, E: Debug> Debug for SharedState<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedState")
            .field("value", &self.value)
            .field("state", &self.state)
            .finish()
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq for SharedState<T, E> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.state == other.state
    }
}

/// Represents a value that may not be done being computed.
/// Once the value has been computed, it will be stored in its shared state.
/// Should be used as a `Future`, and can be `await`ed in async code.
#[derive(Clone)]
pub struct VariableActivation<T, E> {
    /// A slot for the data once it arrives, as well as
    /// the waker to call once a result has been produced.
    pub shared_state: Arc<Mutex<SharedState<T, E>>>,
    /// A reference to the thread that is producing the result.
    /// Dropping this tells the worker that this value no longer requires the outputs of the computation.
    pub producer: Option<TerminationHandle>,
}

impl<T: Clone, E: Clone> VariableActivation<T, E> {
    /// Returns a reference to the shared state of this variable activation.
    pub fn shared_state(&self) -> &Arc<Mutex<SharedState<T, E>>> {
        &self.shared_state
    }

    /// Drops the termination handle reference
    pub fn cancel(&mut self) {
        self.producer = None;
    }

    /// Returns the value of this activation if it is done,
    /// or the latest one that was produced otherwise.
    pub fn latest_value(&self) -> T {
        self.shared_state.lock().unwrap().current_value().clone()
    }
}

impl<T: Debug, E: Debug> Debug for VariableActivation<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shared_state = self
            .shared_state
            .lock()
            .expect("Could not lock shared_state");
        write!(f, "{:?}", shared_state)
    }
}

impl<T, E> From<T> for VariableActivation<T, E> {
    fn from(value: T) -> Self {
        Self {
            shared_state: Arc::new(Mutex::new(SharedState::from(value))),
            producer: None,
        }
    }
}

impl<T: Clone, E: Clone> Future for VariableActivation<T, E> {
    type Output = (T, State<E>);

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        match &shared_state.state {
            // Still waiting for a value
            State::Pending => {
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
            // It is complete, either Ready or Error.
            state => Poll::Ready((shared_state.value.clone(), state.clone())),
        }
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq for VariableActivation<T, E> {
    /// TODO: Avoid deadlocks here?
    fn eq(&self, other: &Self) -> bool {
        let v1 = self.shared_state.lock().expect("Coult not lock st1");
        let v2 = other.shared_state.lock().expect("Coult not lock st2");
        *v1 == *v2
    }
}
