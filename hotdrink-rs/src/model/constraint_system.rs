//! Types for representing constraint systems.
//! This works as a container for components, and provides an API
//! for interacting with them.

use super::{
    activation::Activation,
    component::Component,
    errors::{NoSuchComponent, NoSuchItem},
    undo::{NoMoreRedo, NoMoreUndo, UndoLimit},
    variable::Variable,
};
use crate::{
    event::Event,
    executor::{DummyExecutor, MethodExecutor},
    planner::PlanError,
    solver::SolveError,
};
use std::{collections::HashMap, fmt::Debug};

/// A container for [`Component`]s.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstraintSystem<T> {
    components: HashMap<String, Component<T>>,
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
}

impl<T> Default for ConstraintSystem<T> {
    fn default() -> Self {
        Self {
            components: HashMap::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }
}

impl<T> ConstraintSystem<T> {
    /// Creates a new constraint system with no components.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a component to the constraint system.
    pub fn add_component(&mut self, component: Component<T>) {
        self.components
            .insert(component.name().to_owned(), component);
    }

    /// Removes a component from the constraint system.
    pub fn remove_component(&mut self, name: &str) -> Option<Component<T>> {
        self.components.remove(name)
    }

    /// Get a reference to the selected component.
    pub fn component<'s>(&self, name: &'s str) -> Result<&Component<T>, NoSuchComponent<'s>> {
        self.components.get(name).ok_or(NoSuchComponent(name))
    }

    /// Get a mutable reference to the selected component.
    pub fn component_mut<'s>(
        &mut self,
        name: &'s str,
    ) -> Result<&mut Component<T>, NoSuchComponent<'s>> {
        self.components.get_mut(name).ok_or(NoSuchComponent(name))
    }

    /// Returns an iterator over the [`ConstraintSystem`]'s components.
    pub fn components(&self) -> impl ExactSizeIterator<Item = &Component<T>> {
        self.components.values()
    }

    /// Edits the specified variable's value.
    pub fn edit<'s>(
        &mut self,
        component: &'s str,
        variable: &'s str,
        value: impl Into<T>,
    ) -> Result<(), NoSuchItem<'s>> {
        self.undo_stack.push(component.to_string());
        self.redo_stack.clear();
        let component = self.component_mut(component)?;
        component.edit(variable, value)?;
        Ok(())
    }

    /// Returns the current value of the variable with name `variable` in `component`, if one exists.
    pub fn variable<'a>(
        &self,
        component: &'a str,
        variable: &'a str,
    ) -> Result<&Variable<Activation<T>>, NoSuchItem<'a>> {
        let component = self.component(component)?;
        let variable = component.variable(variable)?;
        Ok(variable)
    }

    /// Returns the current activation of the variable with name `variable` in `component`, if one exists.
    pub fn value<'a>(
        &self,
        component: &'a str,
        variable: &'a str,
    ) -> Result<Activation<T>, NoSuchItem<'a>> {
        let component = self.component(component)?;
        let variable = component.value(variable)?;
        Ok(variable)
    }

    /// Attempts to enforce all constraints in every component that is modified.
    /// If no plan could be found, it will return a [`PlanError`].
    /// This variant lets you specify a thread pool to run methods on.
    pub fn solve(&mut self) -> Result<(), PlanError>
    where
        T: Send + Sync + 'static + Debug,
    {
        log::trace!("update");
        self.par_solve(&DummyExecutor)
    }

    /// Attempts to enforces all constraints in every component that is modified.
    /// If no plan could be found, it will return a [`PlanError`].
    /// This variant lets you specify a thread pool to run methods on.
    pub fn par_solve(&mut self, spawn: &impl MethodExecutor) -> Result<(), PlanError>
    where
        T: Send + Sync + 'static + Debug,
    {
        log::trace!("par_update");
        for component in self.components.values_mut() {
            if component.is_modified() {
                component.par_solve(spawn)?;
            }
        }

        Ok(())
    }

    /// Attempts to enforces all constraints in every component, even if they have not been modified.
    /// If no plan could be found, it will return a [`PlanError`].
    pub fn par_update_always(&mut self, spawn: &mut impl MethodExecutor) -> Result<(), PlanError>
    where
        T: Send + Sync + 'static + Debug,
    {
        for component in self.components.values_mut() {
            component.par_solve(spawn)?;
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
    /// use hotdrink_rs::{model::ConstraintSystem, component, ret, event::{Event, Ready}};
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
    ///     Event::Ready(v) => assert_eq!(v, Ready::Changed(&0)),
    ///     Event::Error(errors) => panic!("{:?}", errors),
    /// });
    /// ```
    pub fn subscribe<'a>(
        &mut self,
        component: &'a str,
        variable: &'a str,
        callback: impl for<'e> Fn(Event<'e, T, SolveError>) + Send + Sync + 'static,
    ) -> Result<(), NoSuchItem<'a>>
    where
        T: 'static,
    {
        log::trace!("Subscribing to {}.{}", component, variable);
        let component = self.component_mut(component)?;
        component
            .subscribe(variable, callback)
            .map_err(NoSuchItem::Variable)
    }

    /// Unsubscribe from a variable in the specified component to avoid receiving further events.
    pub fn unsubscribe<'a>(
        &mut self,
        component: &'a str,
        variable: &'a str,
    ) -> Result<(), NoSuchItem<'a>> {
        log::trace!("Unsubscribing from {}.{}", component, variable);
        let component = self.component_mut(component)?;
        component
            .unsubscribe(variable)
            .map_err(NoSuchItem::Variable)
    }

    /// Pins a variable.
    ///
    /// This adds a stay constraint to the specified variable,
    /// meaning that planning will attempt to avoid modifying it.
    /// The stay constraint can be remove with [`unpin`](#method.unpin).
    pub fn pin<'s>(&mut self, component: &'s str, variable: &'s str) -> Result<(), NoSuchItem<'s>>
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
    pub fn unpin<'s>(&mut self, component: &'s str, variable: &'s str) -> Result<(), NoSuchItem<'s>>
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
        log::trace!("Undoing last change in {}", component.name());
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
        log::trace!("Redoing last change in {}", component.name());
        let _ = component.redo()?;
        self.undo_stack.push(last_redone);
        Ok(())
    }

    /// Sets the undo-limit per component in the system.
    pub fn set_undo_limit(&mut self, limit: UndoLimit) {
        for component in self.components.values_mut() {
            component.set_undo_limit(limit);
        }
    }

    /// Enables the specified constraint.
    pub fn enable_constraint<'a>(
        &mut self,
        component: &'a str,
        constraint: &'a str,
    ) -> Result<(), NoSuchItem<'a>> {
        let component = self.component_mut(component)?;
        component.enable_constraint(constraint)?;
        Ok(())
    }

    /// Disabled the specified constraint.
    pub fn disable_constraint<'a>(
        &mut self,
        component: &'a str,
        constraint: &'a str,
    ) -> Result<(), NoSuchItem<'a>> {
        let component = self.component_mut(component)?;
        component.disable_constraint(constraint)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ConstraintSystem;
    use crate::{
        component,
        event::{Event, Ready},
        ret,
    };

    #[test]
    pub fn constraint_system_test() {
        // Construct the constraint system
        let mut cs: ConstraintSystem<i32> = ConstraintSystem::new();
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
        cs.edit("comp", "a", 7).unwrap();
        assert_eq!(cs.solve(), Ok(()));

        let comp = cs.component_mut("comp").unwrap();
        comp.subscribe("a", |event| {
            if let Event::Ready(v) = event {
                assert_eq!(v, Ready::Changed(&7))
            }
        })
        .unwrap();
        comp.subscribe("b", |event| {
            if let Event::Ready(v) = event {
                assert_eq!(v, Ready::Changed(&0))
            }
        })
        .unwrap();
        comp.subscribe("c", |event| {
            if let Event::Ready(v) = event {
                assert_eq!(v, Ready::Changed(&7))
            }
        })
        .unwrap();
    }
}
