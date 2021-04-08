//! Types for representing constraint systems.
//! This works as a container for components, and provides an API
//! for interacting with them.

use super::{
    component::Component,
    errors::{ApiError, NoSuchComponent},
    solve_error::SolveError,
    spec::PlanError,
    undo_vec::{NoMoreRedo, NoMoreUndo, UndoLimit},
    variable_activation::DoneState,
};
use crate::{
    event::Event,
    thread::{DummyPool, ThreadPool},
};
use std::{collections::HashMap, fmt::Debug, future::Future};

/// A container for `Component`s.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintSystem<T> {
    component_map: HashMap<String, usize>,
    components: Vec<Component<T>>,
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
}

impl<T> Default for ConstraintSystem<T> {
    fn default() -> Self {
        Self {
            component_map: HashMap::new(),
            components: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }
}

impl<T: Debug> ConstraintSystem<T> {
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

    /// Get a reference to the selected component.
    pub fn component<'s>(&self, name: &'s str) -> Result<&Component<T>, NoSuchComponent<'s>> {
        match self.component_map.get(name) {
            Some(&index) => Ok(&self.components[index]),
            None => Err(NoSuchComponent(&name)),
        }
    }

    /// Get a mutable reference to the selected component.
    pub fn component_mut<'s>(
        &mut self,
        name: &'s str,
    ) -> Result<&mut Component<T>, NoSuchComponent<'s>> {
        match self.component_map.get(name) {
            Some(&index) => Ok(&mut self.components[index]),
            None => Err(NoSuchComponent(&name)),
        }
    }

    /// Updates the specified variable to the provided value.
    pub fn set_variable<'s>(
        &mut self,
        component: &'s str,
        variable: &'s str,
        value: T,
    ) -> Result<(), ApiError<'s>> {
        self.undo_stack.push(component.to_string());
        self.redo_stack.clear();
        log::trace!("Variable {}.{} updated to {:?}", component, variable, value);
        let component = self.component_mut(component)?;
        component.set_variable(variable, value)?;
        Ok(())
    }

    /// Returns the current value of the variable with name `var`, if one exists.
    pub fn get_variable<'s>(
        &self,
        component: &'s str,
        variable: &'s str,
    ) -> Result<impl Future<Output = DoneState<T, SolveError>>, ApiError<'s>> {
        let component = self.component(component)?;
        let variable = component.variable(variable)?;
        Ok(variable)
    }

    /// Solves each component in the constraint system,
    /// and increments the generation counter.
    pub fn update(&mut self) -> Result<(), PlanError>
    where
        T: Send + Sync + 'static + Debug,
    {
        self.par_update(&mut DummyPool)
    }

    /// Solve each component as usual, but pass along a thread pool to run computations on.
    /// The callbacks will be called as usual upon completion.
    pub fn par_update(&mut self, spawn: &mut impl ThreadPool) -> Result<(), PlanError>
    where
        T: Send + Sync + 'static + Debug,
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
        T: Send + Sync + 'static + Debug,
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
    /// use hotdrink_rs::{model::ConstraintSystem, component, ret, Event};
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
    ///     Event::Ready(v) => assert_eq!(*v, 0),
    ///     Event::Error(errors) => panic!("{:?}", errors),
    /// });
    /// ```
    pub fn subscribe(
        &mut self,
        component: &str,
        variable: &str,
        callback: impl for<'a> Fn(Event<'a, T, SolveError>) + Send + 'static,
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
    /// The stay constraint can be remove with [`unpin`](#method.unpin).
    pub fn pin<'s>(&mut self, component: &'s str, variable: &'s str) -> Result<(), ApiError<'s>>
    where
        T: 'static,
    {
        let component = self.component_mut(component)?;
        component.pin(variable)?;
        Ok(())
    }

    /// Unpins a variable.
    ///
    /// This removes the stay constraint added by [`pin`](#method.pin).
    pub fn unpin<'s>(&mut self, component: &'s str, variable: &'s str) -> Result<(), ApiError<'s>>
    where
        T: 'static,
    {
        let component = self.component_mut(component)?;
        component.unpin(variable)?;
        Ok(())
    }

    /// Undo the last change of the last modified component.
    pub fn undo(&mut self) -> Result<(), NoMoreUndo> {
        let last_undone = self.undo_stack.pop().ok_or(NoMoreUndo)?;
        let component = self
            .component_mut(&last_undone)
            .expect("Component was removed");
        log::info!("Undoing last change in {}", component.name());
        let _ = component.undo()?;
        self.redo_stack.push(last_undone);
        Ok(())
    }

    /// Redo the last change of the last modified component.
    pub fn redo(&mut self) -> Result<(), NoMoreRedo> {
        let last_redone = self.redo_stack.pop().ok_or(NoMoreRedo)?;
        let component = self
            .component_mut(&last_redone)
            .expect("Component was removed");
        log::info!("Redoing last change in {}", component.name());
        let _ = component.redo()?;
        self.undo_stack.push(last_redone);
        Ok(())
    }

    /// Sets the undo-limit per component in the system.
    pub fn set_undo_limit(&mut self, limit: UndoLimit) {
        for component in &mut self.components {
            component.set_undo_limit(limit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConstraintSystem;
    use crate::{component, event::Event, ret};

    #[test]
    pub fn constraint_system_test() {
        // Construct the constraint system
        let mut cs = ConstraintSystem::new();
        cs.add_component(component! {
            component comp {
                let a: i32 = 0, b: i32 = 0, c: i32 = 0;
                constraint sum {
                    abc(a: &i32, b: &i32) -> [c] = ret![a + b];
                    bca(a: &i32, c: &i32) -> [b] = ret![c - a];
                    cab(b: &i32, c: &i32) -> [a] = ret![c - b];
                }
            }
        });

        // Update a few variable values
        cs.set_variable("comp", "a", 7).unwrap();
        assert_eq!(cs.update(), Ok(()));

        let comp = cs.component_mut("comp").unwrap();
        comp.subscribe("a", |event| {
            if let Event::Ready(v) = event {
                assert_eq!(*v, 7)
            }
        })
        .unwrap();
        comp.subscribe("b", |event| {
            if let Event::Ready(v) = event {
                assert_eq!(*v, 0)
            }
        })
        .unwrap();
        comp.subscribe("c", |event| {
            if let Event::Ready(v) = event {
                assert_eq!(*v, 7)
            }
        })
        .unwrap();
    }
}
