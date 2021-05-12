//! Types for representing a [`Activation`]. That is, the latest (possibly not computed yet) value of a variable.
//! Updating the values of a constraint system will not happen immediately, but the activations will be ready,
//! and act as futures or promises that eventually get the new value.

use crate::{event::Event, executor::TerminationHandle, scheduler::SolveError};
use derivative::Derivative;
use futures::Future;
use std::{
    fmt::Debug,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

/// Data to store while the computation is pending.
#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug, PartialEq, Eq)]
pub struct PendingData<T> {
    previous: Activation<T>,
    dependencies: Vec<Activation<T>>,
}

impl<T> PendingData<T> {
    /// Constructs a new [`PendingData<T>`].
    pub fn new(previous: Activation<T>, dependencies: Vec<Activation<T>>) -> Self {
        Self {
            previous,
            dependencies,
        }
    }
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""), Debug, PartialEq, Eq)]
pub struct ErrorData<T> {
    previous: Activation<T>,
    errors: Vec<SolveError>,
}

impl<T> ErrorData<T> {
    pub fn new(previous: Activation<T>, errors: Vec<SolveError>) -> Self {
        Self { previous, errors }
    }
    pub fn previous(&self) -> &Activation<T> {
        &self.previous
    }
    pub fn errors(&self) -> &Vec<SolveError> {
        &self.errors
    }
}

/// The possible states of a variable's value.
/// It starts off with being pending, and can
/// transition to [`State::Ready`] when its computation succeeds,
/// or [`State::Error`] if the computation fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State<T> {
    /// The value is still being computed.
    Pending(PendingData<T>),
    /// The value was computed successfully.
    Ready(Arc<T>),
    /// The computation of the value failed.
    Error(ErrorData<T>),
}

/// A callback to an [`Event`] sent from a call to [`ConstraintSystem::solve`](crate::model::ConstraintSystem::solve).
pub type EventCallback<T, E> = Arc<Mutex<dyn Fn(Event<'_, T, E>) + Send>>;

/// Contains a slot for a value to be produced,
/// and one for a waker to be called when this happens.
#[derive(Derivative)]
#[derivative(Debug = "transparent", PartialEq, Eq)]
pub struct ActivationInner<T> {
    state: State<T>,
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    waker: Option<Waker>,
}

impl<T> From<T> for ActivationInner<T> {
    fn from(value: T) -> Self {
        Self {
            state: State::Ready(Arc::new(value)),
            waker: None,
        }
    }
}

impl<T> ActivationInner<T> {
    pub fn new(previous: Activation<T>, dependencies: Vec<Activation<T>>) -> Self {
        Self {
            state: State::Pending(PendingData::new(previous, dependencies)),
            waker: None,
        }
    }

    /// Returns a reference to the current state.
    pub fn state(&self) -> &State<T> {
        &self.state
    }

    /// Sets the state to a successful value.
    pub fn set_value(&mut self, value: T) {
        self.state = State::Ready(Arc::new(value));
        self.wake();
    }

    /// Sets the state to a successful value.
    pub fn set_value_arc(&mut self, value: Arc<T>) {
        self.state = State::Ready(value);
        self.wake();
    }

    /// Set the state to a failed value.
    pub fn set_error(&mut self, errors: Vec<SolveError>) {
        match &mut self.state {
            State::Error(previous_errors) => {
                previous_errors.errors.extend(errors);
            }
            State::Pending(pd) => {
                self.state = State::Error(ErrorData::new(pd.previous.clone(), errors));
            }
            _ => panic!("State set to error twice"),
        }
        self.wake();
    }

    /// Returns a mutable reference to the [`Waker`].
    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

/// Represents a value that may not be done being computed.
/// Once the value has been computed, it will be stored in its shared state.
/// Should be used as a `Future`, and can be `await`ed in async code.
#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Activation<T> {
    /// A slot for the data once it arrives, as well as
    /// the waker to call once a result has been produced.
    pub inner: Arc<Mutex<ActivationInner<T>>>,
    /// A handle that when there are no more references to it,
    /// a flag is set so that the computing thread can be cancelled.
    pub producer: Option<TerminationHandle>,
}

impl<T> Activation<T> {
    /// Returns a reference to the shared state of this variable activation.
    pub fn inner(&self) -> &Arc<Mutex<ActivationInner<T>>> {
        &self.inner
    }

    /// Notes disinterest in the result, halting its computation
    /// if nobody else is interested.
    pub fn cancel(&mut self, e: SolveError) {
        let mut inner = self.inner.lock().unwrap();
        // Only set to error if still pending.
        // Otherwise the result would be overwritten.
        if let State::Pending(_) = &inner.state {
            inner.set_error(vec![e]);
        }
        self.producer = None;
    }

    /// Removes the reference to the termination handle.
    pub fn detach(&mut self) {
        self.producer = None;
    }

    /// Returns an activation that will not contribute to keeping the computing thread alive.
    pub fn weak_clone(&self) -> Self {
        let mut clone = self.clone();
        clone.producer = None;
        clone
    }

    /// Clear the error of the activation,
    /// and set the value to the previous successful one.
    pub fn clear_error(&mut self) {
        let opt_old: Option<Activation<T>> = {
            let mut inner = self.inner.lock().unwrap();
            if let State::Error(ed) = &mut inner.state {
                Some(ed.previous().clone())
            } else {
                None
            }
        };
        if let Some(old) = opt_old {
            *self = old;
        }
    }
}

impl<T: Debug> Debug for Activation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shared_state = self.inner.lock().expect("Could not lock shared_state");
        write!(f, "Activation({:?})", shared_state)
    }
}

impl<T> From<T> for Activation<T> {
    fn from(value: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ActivationInner::from(value))),
            producer: None,
        }
    }
}

/// The resulting value of a [`Activation`].
pub type Value<T> = Result<Arc<T>, (Arc<T>, ErrorData<T>)>;

impl<T> Future for Activation<T> {
    type Output = Value<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        match &inner.state {
            // Still waiting for a value
            State::Pending(_) => {
                inner.waker = Some(cx.waker().clone());
                Poll::Pending
            }
            // It is complete, either Ready or Error.
            State::Ready(value) => Poll::Ready(Ok(Arc::clone(value))),
            State::Error(error_data) => {
                let clone: Activation<T> = error_data.previous().clone();
                let mut pinned = Box::pin(clone);
                std::future::Future::poll(pinned.as_mut(), cx)
            }
        }
    }
}

impl<T: PartialEq> PartialEq for Activation<T> {
    /// TODO: Avoid deadlocks here?
    fn eq(&self, other: &Self) -> bool {
        let v1 = self.inner.lock().expect("Could not lock st1");
        let v2 = other.inner.lock().expect("Could not lock st2");
        *v1 == *v2
    }
}

impl<T: Eq> Eq for Activation<T> {}
