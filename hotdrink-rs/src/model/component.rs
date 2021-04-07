//! Types for a [`Component`], an independent subgraph of a constraint system with values and constraints between them.

use super::{
    constraint::Constraint,
    errors::NoSuchVariable,
    filtered_callback::FilteredCallback,
    generation_id::GenerationId,
    generations::{Generations, NoMoreRedo, NoMoreUndo},
    method::Method,
    spec::PlanError,
    variable_activation::{DoneState, State},
};
use super::{solve_error::SolveError, spec::MethodSpec};
use crate::{
    algorithms::{
        hierarchical_planner, priority_adjuster::adjust_priorities, solver,
        OwnedEnforcedConstraint, Vertex,
    },
    event::{Event, SolveEvent, SolveEventWithLoc},
    model::{
        spec::{ComponentSpec, ConstraintSpec},
        variable_activation::VariableActivation,
    },
    thread::{dummy_pool::DummyPool, thread_pool::ThreadPool},
    variable_ranking::{SortRanker, VariableRanker},
};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Write},
    future::Future,
    ops::{Index, IndexMut},
    sync::{Arc, Mutex},
};

/// A collection of variables along with constraints that should be maintained between them.
/// Variables can get new values, values can be retrieved from the component, and the constraints can be enforced.
/// Subscribing to variables will send a notification when the new values are ready.
pub struct Component<T> {
    name: String,
    name_to_index: HashMap<String, usize>,
    callbacks: Arc<Mutex<Vec<FilteredCallback<T, SolveError>>>>,
    activations: Generations<VariableActivation<T, SolveError>>,
    constraints: Vec<Constraint<T>>,
    ranker: SortRanker,
    updated_since_last_solve: HashSet<usize>,
    n_ready: usize,
    current_generation: usize,
    total_generation: usize,
}

impl<T> Clone for Component<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            name_to_index: self.name_to_index.clone(),
            callbacks: self.callbacks.clone(),
            activations: self.activations.clone(),
            constraints: self.constraints.clone(),
            ranker: self.ranker.clone(),
            updated_since_last_solve: self.updated_since_last_solve.clone(),
            n_ready: self.n_ready,
            current_generation: self.current_generation,
            total_generation: self.total_generation,
        }
    }
}

impl<T: Debug> Debug for Component<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Component")
            .field("name", &self.name)
            .field("name_to_idx", &self.name_to_index)
            .field("constraints", &self.constraints)
            .finish()
    }
}

impl<T> Component<T> {
    /// Add a callback to be called when a given variable is updated.
    pub fn subscribe<'s>(
        &mut self,
        variable: &'s str,
        callback: impl Fn(Event<'_, T, SolveError>) + Send + 'static,
    ) -> Result<(), NoSuchVariable<'s>>
    where
        T: 'static,
    {
        if let Some(&index) = self.name_to_index.get(variable) {
            // Call the callback with current variable state
            let activation = &self.activations[index];
            let inner = activation.inner().lock().unwrap();
            match inner.state() {
                State::Pending => callback(Event::Pending),
                State::Ready(value) => callback(Event::Ready(value)),
                State::Error(errors) => callback(Event::Error(errors.clone())),
            }

            // Update the stored callback
            self.callbacks.lock().unwrap()[index].subscribe(callback);

            Ok(())
        } else {
            Err(NoSuchVariable(variable))
        }
    }

    /// Unsubscribe from a variable to avoid receiving further events.
    pub fn unsubscribe<'s>(&mut self, variable: &'s str) -> Result<(), NoSuchVariable<'s>> {
        if let Some(&index) = self.name_to_index.get(variable) {
            self.callbacks.lock().unwrap()[index].unsubscribe();
            Ok(())
        } else {
            Err(NoSuchVariable(variable))
        }
    }

    /// Give a variable a new value.
    pub fn set_variable<'s>(
        &mut self,
        variable: &'s str,
        value: T,
    ) -> Result<(), NoSuchVariable<'s>> {
        if let Some(&idx) = self.name_to_index.get(variable) {
            self.updated_since_last_solve.insert(idx);
            self.ranker.touch(idx);

            // Create a new activation
            self.activations.set(idx, VariableActivation::from(value));
            Ok(())
        } else {
            Err(NoSuchVariable(variable))
        }
    }

    /// Returns the current value of the variable with name `var`, if one exists.
    pub fn variable<'s>(
        &self,
        variable: &'s str,
    ) -> Result<impl Future<Output = DoneState<T, SolveError>>, NoSuchVariable<'s>> {
        let idx = self.variable_index(variable)?;
        Ok(self.activations[idx].clone())
    }

    /// Returns the index of the specified variable, if it exists.
    fn variable_index<'s>(&self, variable: &'s str) -> Result<usize, NoSuchVariable<'s>> {
        match self.name_to_index.get(variable) {
            Some(&index) => Ok(index),
            None => Err(NoSuchVariable(variable)),
        }
    }

    /// Returns a reference to the name of this component.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name of the component.
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    /// Returns a [`Vec<&str>`] of names of variables in this component.
    pub fn variable_names(&self) -> Vec<&str> {
        self.name_to_index.keys().map(String::as_str).collect()
    }

    /// Constructs a new component from a precomputed map from variable names to indices.
    pub fn new_with_map(
        name: String,
        name_to_idx: HashMap<String, usize>,
        values: Vec<T>,
        constraints: Vec<Constraint<T>>,
    ) -> Self {
        // Create map and inverse map
        let mut idx_to_name: HashMap<usize, String> = HashMap::new();
        for (k, v) in &name_to_idx {
            idx_to_name.insert(*v, k.clone());
        }
        // Create new component and set maps
        let mut component = Component::new(name, values, constraints);
        component.name_to_index = name_to_idx;
        component
    }

    /// Solves a component by spawning method activations in parallel,
    /// and updates the futures of values in the constraint system instead.
    pub fn par_update(&mut self, pool: &mut impl ThreadPool) -> Result<(), PlanError>
    where
        T: Send + Sync + 'static + Debug,
    {
        // Rank variables and run planner
        let ranking = self.ranker.ranking();
        let plan = hierarchical_planner(self, &ranking)?;
        self.solve(pool, plan)?;
        Ok(())
    }

    /// Executes `plan` using `pool` in order to enforce all constraints.
    fn solve(
        &mut self,
        pool: &mut impl ThreadPool,
        plan: Vec<OwnedEnforcedConstraint<Method<T>>>,
    ) -> Result<(), PlanError>
    where
        T: Send + Sync + 'static + Debug,
    {
        self.ranker = adjust_priorities(&plan, &self.ranker);

        self.updated_since_last_solve.clear();
        let component_name = self.name().to_owned();

        // Clone the variable information for use in the general callback
        let variable_information_clone = self.callbacks.clone();

        // Set the new callbacks to respond to.
        self.current_generation += 1;
        self.total_generation += 1;
        let generation = GenerationId::new(self.current_generation, self.total_generation);
        for fcb in &mut *self.callbacks.lock().unwrap() {
            fcb.set_target(generation);
        }

        // Solve based on the plan
        solver::par_solve(
            &plan,
            &mut self.activations,
            component_name,
            generation,
            pool,
            move |ge| {
                let mut lock = variable_information_clone.lock().unwrap();
                let fcb = &mut lock[ge.variable()];
                fcb.call(ge);
            },
        )?;

        // Commit changes
        self.activations.commit();

        Ok(())
    }

    /// Pins a variable.
    ///
    /// This adds a stay constraint to the specified variable,
    /// meaning that planning will attempt to avoid modifying it.
    /// The stay constraint can be remove with [`Component::unpin`].
    pub fn pin<'s>(&mut self, variable: &'s str) -> Result<(), NoSuchVariable<'s>>
    where
        T: 'static,
    {
        let idx = self.variable_index(variable)?;
        self.constraints.push(Constraint::new(vec![Method::new(
            "pin".to_owned() + &idx.to_string(),
            vec![idx],
            vec![idx],
            Arc::new(Ok),
        )]));
        Ok(())
    }

    /// Unpins a variable.
    ///
    /// This removes the stay constraint added by [`Component::pin`].
    pub fn unpin<'s>(&mut self, variable: &'s str) -> Result<(), NoSuchVariable<'s>>
    where
        T: 'static,
    {
        let idx = self.variable_index(variable)?;
        self.constraints.drain_filter(|c| {
            c.methods()
                .get(0)
                .map(|m| m.name() == "pin".to_owned() + &idx.to_string())
                .unwrap_or(false)
        });
        Ok(())
    }

    /// Returns true if any variables have been updated since
    /// the last solve, meaning that any constraints may be broken.
    pub fn is_modified(&self) -> bool {
        !self.updated_since_last_solve.is_empty()
    }

    /// Returns the current ranking of variables,
    /// based on when they were last updated.
    pub fn ranking(&self) -> Vec<usize> {
        self.ranker.ranking()
    }

    /// Constructs a string-representation of a graph formatted in the [dot language](https://graphviz.org/doc/info/lang.html).
    /// This can be used for visualization of the constraint graph that the component represents.
    /// This function includes every method in a constraint.
    pub fn to_dot_detailed(&self) -> Result<String, fmt::Error> {
        // Generate map from index to name
        let mut index_to_name = HashMap::new();
        for (k, v) in &self.name_to_index {
            index_to_name.insert(v, k);
        }

        // Start writing dot formatted text
        let mut buffer = String::new();
        writeln!(buffer, "strict digraph {} {{", self.name())?;
        writeln!(buffer, "  rankdir=LR;")?;

        // Set variable shapes to box
        for vi in 0..self.n_variables() {
            if let Some(name) = index_to_name.get(&vi) {
                writeln!(buffer, "  {} [shape=box];", name)?;
            }
        }

        for c in self.constraints() {
            writeln!(buffer, "  subgraph {} {{", c.name())?;
            writeln!(buffer, "    color=gray;")?;
            writeln!(buffer, "    style=filled;")?;
            writeln!(buffer, "    style=rounded;")?;
            writeln!(buffer, "    label={};", c.name())?;
            for m in c.methods() {
                writeln!(
                    buffer,
                    "    {}_{} [label={}];",
                    c.name(),
                    m.name(),
                    m.name()
                )?;
            }
            write!(buffer, "    {{ rank = same; ")?;
            for m in c.methods() {
                write!(buffer, "{}_{}; ", c.name(), m.name())?;
            }
            writeln!(buffer, "}}")?;
            writeln!(buffer, "  }}")?;
        }

        for c in self.constraints() {
            for m in c.methods() {
                // Draw an arrow from input-variable to method
                for i in m.inputs() {
                    let var_name = index_to_name[i];
                    writeln!(
                        buffer,
                        "  {} -> {}_{} [style=dotted];",
                        &var_name,
                        c.name(),
                        m.name()
                    )?;
                }
                // Draw an arrow from method to output-variable
                for o in m.outputs() {
                    let var_name = index_to_name[o];
                    writeln!(buffer, "  {}_{} -> {};", c.name(), m.name(), var_name)?;
                }
            }
        }

        writeln!(buffer, "}}")?;

        Ok(buffer)
    }

    /// Constructs a string-representation of a graph formatted in the [dot language](https://graphviz.org/doc/info/lang.html).
    /// This can be used for visualization of the constraint graph that the component represents.
    /// This function only includes constraints, and not methods.
    pub fn to_dot_simple(&self) -> Result<String, fmt::Error> {
        // Generate map from index to name
        let mut index_to_name = HashMap::new();
        for (k, v) in &self.name_to_index {
            index_to_name.insert(v, k);
        }

        // Start writing dot formatted text
        let mut buffer = String::new();
        writeln!(buffer, "strict graph {} {{", self.name())?;

        // Set variable shapes to box
        for vi in 0..self.n_variables() {
            if let Some(name) = index_to_name.get(&vi) {
                writeln!(buffer, "  {} [shape=box];", name)?;
            }
        }

        for c in self.constraints() {
            for v in c.variables() {
                let var_name = index_to_name.get(v).unwrap();
                writeln!(buffer, "    {} -- {};", c.name(), var_name)?;
            }
        }

        writeln!(buffer, "}}")?;

        Ok(buffer)
    }

    fn notify(&self, callbacks: &[FilteredCallback<T, SolveError>]) {
        for (vi, v) in callbacks.iter().enumerate() {
            let va = &self.activations[vi];
            let inner = va.inner().lock().unwrap();
            let event = match inner.state() {
                State::Ready(value) => SolveEvent::Ready(Arc::clone(value)),
                State::Error(errors) => SolveEvent::Error(errors.clone()),
                State::Pending => SolveEvent::Pending,
            };
            v.call(SolveEventWithLoc::new(
                vi,
                GenerationId::new(self.current_generation, self.total_generation),
                event,
            ));
        }
    }

    /// Jumps back to the previous set/update call.
    pub fn undo(&mut self) -> Result<(), NoMoreUndo> {
        // Lock callbacks and which events to respond to
        let mut callbacks = self.callbacks.lock().unwrap();
        self.activations.undo()?;
        self.current_generation -= 1;
        self.total_generation += 1;
        // Update target to accept events from this `notify`-call.
        for fcb in callbacks.iter_mut() {
            fcb.set_target(GenerationId::new(
                self.current_generation,
                self.total_generation,
            ));
        }
        self.notify(&callbacks);

        Ok(())
    }

    /// Jumps forward to the next set/update call.
    pub fn redo(&mut self) -> Result<(), NoMoreRedo> {
        // Lock callbacks and which events to respond to
        let mut callbacks = self.callbacks.lock().unwrap();
        self.activations.redo()?;
        self.current_generation += 1;
        self.total_generation += 1;
        // Update target to accept events from this `notify`-call.
        for fcb in callbacks.iter_mut() {
            fcb.set_target(GenerationId::new(
                self.current_generation,
                self.total_generation,
            ));
        }
        self.notify(&callbacks);

        Ok(())
    }
}

impl<T> ComponentSpec for Component<T> {
    type Value = T;
    type Variable = VariableActivation<T, SolveError>;
    type Constraint = Constraint<T>;

    fn new(
        name: String,
        values: Vec<impl Into<Self::Variable>>,
        constraints: Vec<Self::Constraint>,
    ) -> Self {
        let n_variables = values.len();
        let values = Generations::new(values.into_iter().map(|v| v.into()).collect());
        Self {
            name,
            name_to_index: HashMap::new(),
            activations: values,
            callbacks: Arc::new(Mutex::new(vec![FilteredCallback::new(); n_variables])),
            constraints,
            ranker: VariableRanker::of_size(n_variables),
            updated_since_last_solve: (0..n_variables).collect(),
            n_ready: n_variables,
            current_generation: 0,
            total_generation: 0,
        }
    }

    fn update(&mut self) -> Result<(), PlanError>
    where
        T: Send + Sync + 'static + Debug,
    {
        self.par_update(&mut DummyPool)
    }

    fn n_variables(&self) -> usize {
        self.activations.n_variables()
    }

    fn variables(&self) -> Vec<&Self::Variable> {
        self.activations.values()
    }

    fn get(&self, i: usize) -> &Self::Variable {
        &self.activations[i]
    }

    fn set(&mut self, i: usize, value: impl Into<Self::Value>) {
        self.activations
            .set(i, VariableActivation::from(value.into()));
    }

    fn constraints(&self) -> &[Self::Constraint] {
        &self.constraints
    }

    fn constraints_mut(&mut self) -> &mut [Self::Constraint] {
        &mut self.constraints
    }

    fn push(&mut self, constraint: Self::Constraint) {
        self.constraints.push(constraint)
    }

    fn pop(&mut self) -> Option<Self::Constraint> {
        self.constraints.pop()
    }

    fn name_to_idx(&self, name: &str) -> Option<usize> {
        self.name_to_index.get(name).copied()
    }

    fn remove_constraint(&mut self, idx: usize) -> Self::Constraint {
        self.constraints.remove(idx)
    }
}

impl<T> Index<&str> for Component<T> {
    type Output = Constraint<T>;

    /// Returns a reference to the constraint with the given name.
    ///
    /// # Panics
    ///
    /// Panics if the constraint is not present in the `Component`.
    fn index(&self, index: &str) -> &Self::Output {
        for constraint in &self.constraints {
            if constraint.name() == index {
                return constraint;
            }
        }

        panic!("No constraint named {}", index)
    }
}

impl<T> IndexMut<&str> for Component<T> {
    /// Returns a mutable reference to the constraint with the given name.
    ///
    /// # Panics
    ///
    /// Panics if the constraint is not present in the `Component`.
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        for constraint in &mut self.constraints {
            if constraint.name() == index {
                return constraint;
            }
        }

        panic!("No constraint named {}", index)
    }
}

impl<T: PartialEq> PartialEq for Component<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.name_to_index == other.name_to_index
            && self.activations == other.activations
            && self.constraints == other.constraints
            && self.ranker == other.ranker
            && self.updated_since_last_solve == other.updated_since_last_solve
    }
}

#[cfg(test)]
mod tests {
    use super::Component;
    use crate::{
        examples::components::numbers::sum,
        model::{spec::ComponentSpec, variable_activation::VariableActivation},
        thread::dummy_pool::DummyPool,
    };

    #[test]
    fn solve_sum() {
        // Construct component
        let mut component: Component<i32> = sum();

        assert_eq!(
            &component.variables(),
            &[
                &VariableActivation::from(0),
                &VariableActivation::from(0),
                &VariableActivation::from(0)
            ]
        );

        // Update a to 3
        component.set_variable("a", 3).unwrap();
        component.par_update(&mut DummyPool).unwrap();

        assert_eq!(
            &component.variables(),
            &[
                &VariableActivation::from(3),
                &VariableActivation::from(0),
                &VariableActivation::from(3)
            ]
        );

        // Update c to 2
        component.set_variable("c", 2).unwrap();
        component.update().unwrap();

        assert_eq!(
            &component.variables(),
            &[
                &VariableActivation::from(3),
                &VariableActivation::from(-1),
                &VariableActivation::from(2)
            ]
        );
    }

    #[test]
    fn pin_unpin() {
        // Construct component
        let mut component: Component<i32> = sum();
        let val1 = 3;

        // Pin c, which has the lowest priority
        component.pin("c").unwrap();
        component.set_variable("a", val1).unwrap();
        component.update().unwrap();

        // It should pick b, since it is not pinned and has a lower priority than a
        assert_eq!(
            &component.variables(),
            &[
                &VariableActivation::from(val1),
                &VariableActivation::from(-val1),
                &VariableActivation::from(0)
            ]
        );

        let val2 = 5;

        // Unpin c
        component.unpin("c").unwrap();
        component.set_variable("a", val2).unwrap();
        component.update().unwrap();

        // It should pick c, since c is no longer pinned
        assert_eq!(
            &component.variables(),
            &[
                &VariableActivation::from(val2),
                &VariableActivation::from(-val1),
                &VariableActivation::from(val2 - val1)
            ]
        );
    }

    #[test]
    fn undo_redo_works() {
        let mut component: Component<i32> = sum();

        // Perform change and update
        component.set_variable("a", 3).unwrap();
        component.update().unwrap();

        // Verify that change happened
        assert_eq!(
            &component.variables(),
            &[
                &VariableActivation::from(3),
                &VariableActivation::from(0),
                &VariableActivation::from(3)
            ]
        );

        // Undo change
        assert_eq!(component.undo(), Ok(()));

        // Verify that change was undone
        assert_eq!(
            &component.variables(),
            &[
                &VariableActivation::from(0),
                &VariableActivation::from(0),
                &VariableActivation::from(0)
            ]
        );

        // Redo change
        assert_eq!(component.redo(), Ok(()));

        // Verify that change was redone
        assert_eq!(
            &component.variables(),
            &[
                &VariableActivation::from(3),
                &VariableActivation::from(0),
                &VariableActivation::from(3)
            ]
        );
    }
}
