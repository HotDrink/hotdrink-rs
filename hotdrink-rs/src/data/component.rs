//! Types for a [`Component`], an independent subgraph of a constraint system with values and constraints between them.

use super::{
    constraint::Constraint,
    method::Method,
    traits::PlanError,
    variable_activation::State,
    variable_information::{Status, VariableInfo},
};
use super::{solve_error::SolveError, traits::MethodSpec};
use crate::{
    algorithms::{
        hierarchical_planner::{self, OwnedEnforcedConstraint, Vertex},
        priority_adjuster::adjust_priorities,
        solver,
    },
    data::{
        traits::{ComponentSpec, ConstraintSpec},
        variable_activation::VariableActivation,
    },
    event::Event,
    thread::{dummy_pool::DummyPool, thread_pool::ThreadPool},
    variable_ranking::{SortRanker, VariableRanker},
};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Write},
    ops::{Index, IndexMut},
    sync::{Arc, Mutex},
};

/// A callback that responds some input `T`.
pub type GeneralCallback<T> = Arc<Mutex<dyn Fn(T) + Send>>;

/// A collection of variables along with constraints that should be maintained between them.
/// Variables can get new values, values can be retrieved from the component, and the constraints can be enforced.
/// Subscribing to variables will send a notification when the new values are ready.
#[derive(Clone)]
pub struct Component<T> {
    name: String,
    name_to_index: HashMap<String, usize>,
    variable_information: Arc<Mutex<Vec<VariableInfo<T, SolveError>>>>,
    variable_activations: Vec<VariableActivation<T, SolveError>>,
    constraints: Vec<Constraint<T>>,
    ranker: SortRanker,
    updated_since_last_solve: HashSet<usize>,
    n_ready: usize,
    generation: usize,
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

/// Errors that can happen in a component.
#[derive(Copy, Clone, Debug)]
pub enum ComponentError {
    /// No variable of the specified name exists.
    NoSuchVariable,
}

impl<T: Clone> Component<T> {
    /// Add a callback to be called when a given variable is updated.
    pub fn subscribe(
        &mut self,
        variable: &str,
        callback: impl Fn(Event<T, SolveError>) + Send + 'static,
    ) -> Result<(), ComponentError>
    where
        T: 'static,
    {
        if let Some(&index) = self.name_to_index.get(variable) {
            // Call the callback with current variable state
            let activation = &self.variable_activations[index];
            let shared_state = activation.shared_state();
            let shared_state = shared_state.lock().unwrap();
            let value = shared_state.current_value();
            let state = shared_state.get_state();
            match state {
                State::Pending => callback(Event::Pending),
                State::Ready => callback(Event::Ready(value.clone())),
                State::Error(errors) => callback(Event::Error(errors.clone())),
            }

            // Update the stored callback
            self.variable_information.lock().unwrap()[index].subscribe(callback);

            Ok(())
        } else {
            Err(ComponentError::NoSuchVariable)
        }
    }

    /// Unsubscribe from a variable to avoid receiving further events.
    pub fn unsubscribe(&mut self, variable: &str) -> Result<(), ComponentError> {
        if let Some(&index) = self.name_to_index.get(variable) {
            self.variable_information.lock().unwrap()[index].unsubscribe();
            Ok(())
        } else {
            Err(ComponentError::NoSuchVariable)
        }
    }

    /// Give a variable a new value.
    pub fn set_variable(&mut self, var: &str, value: T) -> Result<(), ComponentError> {
        if let Some(&idx) = self.name_to_index.get(var) {
            self.updated_since_last_solve.insert(idx);
            self.ranker.touch(idx);

            // Create a new activation
            self.variable_activations[idx] = VariableActivation::from(value);
            self.variable_information.lock().unwrap()[idx].set_generation(self.generation);
            Ok(())
        } else {
            Err(ComponentError::NoSuchVariable)
        }
    }

    /// Returns the current value of the variable with name `var`, if one exists.
    pub fn get_variable(&self, var: &str) -> Option<T> {
        self.name_to_index
            .get(var)
            .map(|&idx| self.variable_activations[idx].latest_value())
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
        T: Send + 'static + Debug,
    {
        // Rank variables and run planner
        let ranking = self.ranker.ranking();
        let plan = hierarchical_planner::hierarchical_planner(self, &ranking)?;
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
        T: Send + 'static + Debug,
    {
        self.ranker = adjust_priorities(&plan, &self.ranker);

        self.updated_since_last_solve.clear();
        let component_name = self.name().to_owned();

        // Clone, but cancel the inner activation to avoid two references
        let variable_information_clone = self.variable_information.clone();

        // Solve based on the plan
        solver::par_solve(
            &plan,
            &mut self.variable_activations,
            component_name,
            self.generation,
            pool,
            move |ge| {
                let mut lock = variable_information_clone.lock().unwrap();
                let variable_info = &mut lock[ge.variable()];
                variable_info.call_callback(ge);
            },
        )?;

        self.generation += 1;

        Ok(())
    }

    /// Pins a variable.
    ///
    /// This adds a stay constraint to the specified variable,
    /// meaning that planning will attempt to avoid modifying it.
    /// The stay constraint can be remove with [`Component::unpin`].
    pub fn pin(&mut self, var: &str)
    where
        T: 'static,
    {
        let idx = self.name_to_index[var];
        self.constraints.push(Constraint::new(vec![Method::new(
            "pin".to_owned() + &idx.to_string(),
            vec![idx],
            vec![idx],
            Arc::new(Ok),
        )]));
    }

    /// Unpins a variable.
    ///
    /// This removes the stay constraint added by [`Component::pin`].
    pub fn unpin(&mut self, var: &str)
    where
        T: 'static,
    {
        let idx = self.name_to_index[var];
        self.constraints.drain_filter(|c| {
            c.methods()
                .get(0)
                .map(|m| m.name() == "pin".to_owned() + &idx.to_string())
                .unwrap_or(false)
        });
    }

    /// Returns true if any variables have been updated since
    /// the last solve, meaning that any constraints may be broken.
    pub fn is_modified(&self) -> bool {
        !self.updated_since_last_solve.is_empty()
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
}

impl<T: Clone> ComponentSpec for Component<T> {
    type Value = T;
    type Variable = VariableActivation<T, SolveError>;
    type Constraint = Constraint<T>;

    fn new(
        name: String,
        values: Vec<impl Into<Self::Variable>>,
        constraints: Vec<Self::Constraint>,
    ) -> Self {
        let n_variables = values.len();
        let values = values.into_iter().map(|v| v.into()).collect();
        Self {
            name,
            name_to_index: HashMap::new(),
            variable_activations: values,
            variable_information: Arc::new(Mutex::new(vec![
                VariableInfo::new(Status::Ready);
                n_variables
            ])),
            constraints,
            ranker: VariableRanker::of_size(n_variables),
            updated_since_last_solve: (0..n_variables).collect(),
            n_ready: n_variables,
            generation: 0,
        }
    }

    fn update(&mut self) -> Result<(), PlanError>
    where
        T: Send + 'static + Debug,
    {
        self.par_update(&mut DummyPool)
    }

    fn n_variables(&self) -> usize {
        self.variable_activations.len()
    }

    fn variables(&self) -> &[Self::Variable] {
        &self.variable_activations
    }

    fn get(&self, i: usize) -> &Self::Variable {
        &self.variable_activations[i]
    }

    fn set(&mut self, i: usize, value: impl Into<Self::Value>) {
        self.variable_activations[i] = VariableActivation::from(value.into());
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

impl<T: Clone> Index<&str> for Component<T> {
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

impl<T: Clone> IndexMut<&str> for Component<T> {
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
            && self.variable_activations == other.variable_activations
            && self.constraints == other.constraints
            && self.ranker == other.ranker
            && self.updated_since_last_solve == other.updated_since_last_solve
    }
}

#[cfg(test)]
mod tests {
    use super::Component;
    use crate::{
        data::{traits::ComponentSpec, variable_activation::VariableActivation},
        examples::components::numbers::sum,
        thread::dummy_pool::DummyPool,
    };

    #[test]
    fn solve_sum() {
        // Construct component
        let mut component: Component<i32> = sum();

        assert_eq!(
            component.variables(),
            &[
                VariableActivation::from(0),
                VariableActivation::from(0),
                VariableActivation::from(0)
            ]
        );

        // Update a to 3
        component.set_variable("a", 3).unwrap();
        component.par_update(&mut DummyPool).unwrap();

        assert_eq!(
            component.variables(),
            &[
                VariableActivation::from(3),
                VariableActivation::from(0),
                VariableActivation::from(3)
            ]
        );

        // Update c to 2
        component.set_variable("c", 2).unwrap();
        component.update().unwrap();

        assert_eq!(
            component.variables(),
            &[
                VariableActivation::from(3),
                VariableActivation::from(-1),
                VariableActivation::from(2)
            ]
        );
    }

    #[test]
    fn pin_unpin() {
        // Construct component
        let mut component: Component<i32> = sum();
        let val1 = 3;

        // Pin c, which has the lowest priority
        component.pin("c");
        component.set_variable("a", val1).unwrap();
        component.update().unwrap();

        // It should pick b, since it is not pinned and has a lower priority than a
        assert_eq!(
            component.variables(),
            &[
                VariableActivation::from(val1),
                VariableActivation::from(-val1),
                VariableActivation::from(0)
            ]
        );

        let val2 = 5;

        // Unpin c
        component.unpin("c");
        component.set_variable("a", val2).unwrap();
        component.update().unwrap();

        // It should pick c, since c is no longer pinned
        assert_eq!(
            component.variables(),
            &[
                VariableActivation::from(val2),
                VariableActivation::from(-val1),
                VariableActivation::from(val2 - val1)
            ]
        );
    }
}
