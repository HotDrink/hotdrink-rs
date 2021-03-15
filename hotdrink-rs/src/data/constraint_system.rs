//! Types for representing constraint systems.
//! This works as a container for components, and provides an API
//! for interacting with them.

use super::{component::Component, solve_error::SolveError, traits::PlanError};
use crate::{
    event::Event,
    thread::{dummy_pool::DummyPool, thread_pool::ThreadPool},
};
use std::{collections::HashMap, fmt::Debug};

/// A container for `Component`s.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintSystem<T> {
    component_map: HashMap<String, usize>,
    components: Vec<Component<T>>,
}

impl<T> Default for ConstraintSystem<T> {
    fn default() -> Self {
        Self {
            component_map: HashMap::new(),
            components: Vec::new(),
        }
    }
}

impl<T: Clone + Debug> ConstraintSystem<T> {
    /// Creates a new constraint system with no components.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a component to the constraint system.
    pub fn add_component(&mut self, component: Component<T>) {
        let index = self.components.len();
        self.component_map
            .insert(component.name().to_owned(), index);
        self.components.push(component);
    }

    /// Get a reference to the selected component
    pub fn get_component(&self, name: &str) -> &Component<T> {
        let index = self.component_map[name];
        &self.components[index]
    }

    /// Updates the specified variable to the provided value.
    pub fn set_variable(&mut self, component: &str, variable: &str, value: T) {
        log::debug!("Variable {}.{} updated to {:?}", component, variable, value);
        let index = self.component_map[component];
        self.components[index]
            .set_variable(variable, value)
            .unwrap_or_else(|_| panic!("No such variable: {}.{}", component, variable));
    }

    /// Returns the current value of the variable with name `var`, if one exists.
    pub fn get_variable(&self, component: &str, variable: &str) -> Option<T> {
        let index = self.component_map[component];
        self.components[index].get_variable(variable)
    }

    /// Solves each component in the constraint system,
    /// and increments the generation counter.
    pub fn update(&mut self) -> Result<(), PlanError>
    where
        T: Send + 'static + Debug,
    {
        self.par_update(&mut DummyPool)
    }

    /// Solve each component as usual, but pass along a thread pool to run computations on.
    /// The callbacks will be called as usual upon completion.
    pub fn par_update(&mut self, spawn: &mut impl ThreadPool) -> Result<(), PlanError>
    where
        T: Send + 'static + Debug,
    {
        for component in &mut self.components {
            if component.is_modified() {
                component.par_update(spawn)?;
            }
        }

        Ok(())
    }

    /// Attempts to enforces all constraints in every component.
    /// If no plan could be found, it will return a [`PlanError`].
    pub fn par_update_always(&mut self, spawn: &mut impl ThreadPool) -> Result<(), PlanError>
    where
        T: Send + 'static + Debug,
    {
        for component in &mut self.components {
            component.par_update(spawn)?;
        }

        Ok(())
    }

    /// Attaches a callback to a variable in a component, to be called when its status changes.
    ///
    /// The events sent to the callback will be sent in an order matching an increase in generation.
    /// A pending event will always be sent before either ready or error, and a value from an earlier
    /// generation will never appear after one from a later generation.
    /// This ensures that the event always sends the most up-to-date value.
    ///
    /// # Examples
    /// ```rust
    /// use hotdrink_rs::{data::constraint_system::ConstraintSystem, component, ret, event::Event};
    /// let component = component! {
    ///     component Comp {
    ///         let x: i32 = 0, y: i32 = 0;
    ///         constraint Eq {
    ///             x_to_y(x: &i32) -> [y] = ret![*x];
    ///             y_to_x(y: &i32) -> [x] = ret![*y];
    ///         }
    ///     }
    /// };
    /// let mut cs = ConstraintSystem::new();
    /// cs.add_component(component);
    /// cs.subscribe("Comp", "x", |e| match e {
    ///     Event::Pending => {}
    ///     Event::Ready(v) => assert_eq!(v, 0),
    ///     Event::Error(errors) => panic!(errors),
    /// });
    /// ```
    pub fn subscribe(
        &mut self,
        component: &str,
        variable: &str,
        callback: impl Fn(Event<T, SolveError>) + Send + 'static,
    ) where
        T: 'static,
    {
        let index = self.component_map[component];
        self.components[index]
            .subscribe(variable, callback)
            .expect("Could not subscribe");
    }

    /// Unsubscribe from a variable in the specified component to avoid receiving further events.
    pub fn unsubscribe(&mut self, component: &str, variable: &str) {
        let index = self.component_map[component];
        self.components[index]
            .unsubscribe(variable)
            .expect("Could not unsubscribe");
    }

    /// Pins a variable.
    ///
    /// This adds a stay constraint to the specified variable,
    /// meaning that planning will attempt to avoid modifying it.
    /// The stay constraint can be remove with [`ConstraintSystem::unpin`].
    pub fn pin(&mut self, component: &str, variable: &str)
    where
        T: 'static,
    {
        let index = self.component_map[component];
        self.components[index].pin(variable);
    }

    /// Unpins a variable.
    ///
    /// This removes the stay constraint added by [`ConstraintSystem::pin`].
    pub fn unpin(&mut self, component: &str, variable: &str)
    where
        T: 'static,
    {
        let index = self.component_map[component];
        self.components[index].unpin(variable);
    }
}

#[cfg(test)]
mod tests {
    use super::ConstraintSystem;
    use crate::{
        component, ret,
        thread::{
            dummy_pool::DummyPool,
            thread_pool::{TerminationStrategy, ThreadPool},
        },
    };

    #[test]
    pub fn constraint_system_test() {
        let mut tp = DummyPool::new(0, TerminationStrategy::UnusedResultAndNotDone).unwrap();
        // Construct the constraint system
        let mut cs = ConstraintSystem::new();
        cs.add_component(component! {
            component comp {
                let a: i32 = 0, b: i32 = 0, c: i32 = 0;
                constraint sum {
                    abc(a: &i32, b: &i32) -> [c] = ret![a + b];
                    bca(b: &i32, c: &i32) -> [a] = ret![b + c];
                    cab(c: &i32, a: &i32) -> [b] = ret![c + a];
                }
            }
        });

        // Update a few variable values
        cs.set_variable("comp", "a", 7);
        assert_eq!(cs.par_update(&mut tp), Ok(()));

        // TODO: Replace this test?
        // cs.listen(Arc::new(Mutex::new(|e: Notification| {
        //     if let EventData::Ready(v) = &e.data_ref() {
        //         match e.identifier().variable() {
        //             "a" => assert_eq!(v, &7),
        //             "b" => assert_eq!(v, &0),
        //             "c" => assert_eq!(v, &7),
        //             _ => panic!("No such variable"),
        //         }
        //     }
        // })));
    }
}
