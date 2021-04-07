//! Types for representing a [`VariableActivation`]. That is, the latest (possibly not computed yet) value of a variable.
//! Updating the values of a constraint system will not happen immediately, but the activations will be ready,
//! and act as futures or promises that eventually get the new value.

use crate::{event::Event, thread::TerminationHandle};
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
pub enum State<T, E> {
    /// The value is still being computed.
    Pending,
    /// The value was computed successfully.
    Ready(Arc<T>),
    /// The computation of the value failed.
    Error(Vec<E>),
}

impl<T, E> Default for State<T, E> {
    fn default() -> Self {
        Self::Pending
    }
}

/// A callback to an [`Event`] sent from a call to [`ConstraintSystem::update`](crate::ConstraintSystem::update).
pub type EventCallback<T, E> = Arc<Mutex<dyn Fn(Event<'_, T, E>) + Send>>;

/// Contains a slot for a value to be produced,
/// and one for a waker to be called when this happens.
pub struct VariableActivationInner<T, E> {
    state: State<T, E>,
    waker: Option<Waker>,
}

impl<T, E> Default for VariableActivationInner<T, E> {
    fn default() -> Self {
        Self {
            state: State::Pending,
            waker: None,
        }
    }
}

impl<T, E> VariableActivationInner<T, E> {
    /// Constructs a new [`VariableActivationInner`] with no value.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the current state.
    pub fn state(&self) -> &State<T, E> {
        &self.state
    }

    /// Set the state to [`State::Pending`].
    pub fn set_pending(&mut self) {
        self.state = State::Pending;
    }

    /// Sets the state to a successful value.
    pub fn set_value(&mut self, value: T) {
        self.state = State::Ready(Arc::new(value));
    }

    /// Sets the state to a successful value.
    pub fn set_value_arc(&mut self, value: Arc<T>) {
        self.state = State::Ready(value);
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

impl<T, E> From<T> for VariableActivationInner<T, E> {
    fn from(value: T) -> Self {
        Self {
            state: State::Ready(Arc::new(value)),
            waker: None,
        }
    }
}

impl<T: Debug, E: Debug> Debug for VariableActivationInner<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedState")
            .field("state", &self.state)
            .finish()
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq for VariableActivationInner<T, E> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

/// Represents a value that may not be done being computed.
/// Once the value has been computed, it will be stored in its shared state.
/// Should be used as a `Future`, and can be `await`ed in async code.
pub struct VariableActivation<T, E> {
    /// A slot for the data once it arrives, as well as
    /// the waker to call once a result has been produced.
    pub inner: Arc<Mutex<VariableActivationInner<T, E>>>,
    /// A reference to the thread that is producing the result.
    /// Dropping this tells the worker that this value no longer requires the outputs of the computation.
    pub producer: Option<TerminationHandle>,
}

impl<T, E> Clone for VariableActivation<T, E> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            producer: self.producer.clone(),
        }
    }
}

impl<T, E> VariableActivation<T, E> {
    /// Returns a reference to the shared state of this variable activation.
    pub fn inner(&self) -> &Arc<Mutex<VariableActivationInner<T, E>>> {
        &self.inner
    }

    /// Drops the termination handle reference
    pub fn cancel(&mut self, e: E) {
        let mut inner = self.inner.lock().unwrap();
        // Only set to cancelled if no value was computed in time
        if let State::Pending = inner.state {
            inner.state = State::Error(vec![e]);
        }
        self.producer = None;
    }
}

impl<T: Debug, E: Debug> Debug for VariableActivation<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shared_state = self.inner.lock().expect("Could not lock shared_state");
        write!(f, "{:?}", shared_state)
    }
}

impl<T, E> From<T> for VariableActivation<T, E> {
    fn from(value: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(VariableActivationInner::from(value))),
            producer: None,
        }
    }
}

/// Similar to [`State`], but this one can no longer be pending.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DoneState<T, E> {
    /// The computation succeeded.
    Ready(Arc<T>),
    /// The computation failed.
    Error(Vec<E>),
}

impl<T, E: Clone> Future for VariableActivation<T, E> {
    type Output = DoneState<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        match &inner.state {
            // Still waiting for a value
            State::Pending => {
                inner.waker = Some(cx.waker().clone());
                Poll::Pending
            }
            // It is complete, either Ready or Error.
            State::Ready(value) => Poll::Ready(DoneState::Ready(Arc::clone(value))),
            State::Error(errors) => Poll::Ready(DoneState::Error(errors.clone())),
        }
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq for VariableActivation<T, E> {
    /// TODO: Avoid deadlocks here?
    fn eq(&self, other: &Self) -> bool {
        let v1 = self.inner.lock().expect("Coult not lock st1");
        let v2 = other.inner.lock().expect("Coult not lock st2");
        *v1 == *v2
    }
}
