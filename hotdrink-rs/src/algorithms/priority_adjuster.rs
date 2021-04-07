//! Adjust priorities of variables according to a plan and the previous priorities.
//! This is to avoid unexpected behavior where variables are updated in
//! unexpected ways (not according to the most recent input).

use std::{cmp::Ordering, collections::BinaryHeap};

use super::hierarchical_planner::Vertex;
use crate::variable_ranking::VariableRanker;

#[derive(Eq, Debug)]
struct CompareByPriority<'a> {
    id: usize,
    priorities: &'a [usize],
}

impl<'a> CompareByPriority<'a> {
    fn new(id: usize, priorities: &'a [usize]) -> Self {
        Self { id, priorities }
    }
}

impl PartialEq for CompareByPriority<'_> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl PartialOrd for CompareByPriority<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CompareByPriority<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priorities[self.id].cmp(&other.priorities[other.id])
    }
}

/// Sometimes, when performing edits, variable priorities can
/// end up in such a way that methods that are not downstream from
/// the edit are executed.
/// This may be confusing for the user, as a variable will receive
/// a new value that has nothing to do with the new input.
pub fn adjust_priorities<M: Vertex, R: VariableRanker>(plan: &[M], ranker: &R) -> R {
    // Initialize variables to keep track of state
    let n_variables = ranker.size();
    let mut method_in_degrees = vec![0; plan.len()];
    let mut variable_in_degree = vec![0; n_variables];
    let mut variable_to_method = vec![Vec::new(); n_variables];

    // Count in-degrees
    for (mi, satisfied_constraint) in plan.iter().enumerate() {
        // Count in-degrees of methods
        let method = satisfied_constraint;
        let inputs = method.inputs();
        method_in_degrees[mi] = inputs.len();
        // Create map from variable to methods that read from it
        for &i in inputs {
            variable_to_method[i].push(mi);
        }
        // Count in-degree of variables
        for &o in method.outputs() {
            variable_in_degree[o] += 1;
        }
    }

    // Create priority lookup
    let mut priorities = vec![0; n_variables];
    let old_ranking = ranker.ranking();
    for (vi, &priority) in old_ranking.iter().rev().enumerate() {
        priorities[vi] = priority;
    }

    let mut pq = BinaryHeap::new();

    // Add initial values to priority queue
    for (vi, &in_degree) in variable_in_degree.iter().enumerate() {
        if in_degree == 0 {
            pq.push(CompareByPriority::new(vi, &priorities));
        }
    }

    // Create new priority order
    let mut new_order = Vec::new();
    while let Some(current) = pq.pop() {
        new_order.push(current.id);
        for &m in &variable_to_method[current.id] {
            method_in_degrees[m] -= 1;
            if method_in_degrees[m] == 0 {
                for &v in plan[m].outputs() {
                    variable_in_degree[v] -= 1;
                    if variable_in_degree[v] == 0 {
                        pq.push(CompareByPriority::new(v, &priorities));
                    }
                }
            }
        }
    }

    // Create the new ranker
    let mut new_ranker: R = VariableRanker::of_size(ranker.size());
    for v in new_order.into_iter().rev() {
        new_ranker.touch(v);
    }
    new_ranker
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{adjust_priorities, CompareByPriority};
    use crate::{
        algorithms::hierarchical_planner::{hierarchical_planner, OwnedEnforcedConstraint},
        model::*,
        variable_ranking::{SortRanker, VariableRanker},
    };

    #[test]
    fn compare_by_priority_eq() {
        let priority = [0];
        let a = CompareByPriority::new(0, &priority);
        let b = CompareByPriority::new(0, &priority);
        assert_eq!(a, b, "a should be equal to b");
    }

    #[test]
    fn compare_by_priority_lt() {
        let priority = [0, 1];
        let a = CompareByPriority::new(0, &priority);
        let b = CompareByPriority::new(1, &priority);
        assert!(a < b, "a should be less than b");

        let priority = [1, 0];
        let a = CompareByPriority::new(0, &priority);
        let b = CompareByPriority::new(1, &priority);
        assert!(b < a, "b should be less than a");
    }

    #[test]
    fn compare_by_priority_gt() {
        let priority = [0, 1];
        let a = CompareByPriority::new(0, &priority);
        let b = CompareByPriority::new(1, &priority);
        assert!(b > a, "b should be greater than a");

        let priority = [1, 0];
        let a = CompareByPriority::new(0, &priority);
        let b = CompareByPriority::new(1, &priority);
        assert!(a > b, "a should be greater than b");
    }

    fn dummy<T: 'static>(
        constraint: &str,
        method: &str,
        inputs: &[usize],
        outputs: &[usize],
    ) -> OwnedEnforcedConstraint<Method<T>> {
        OwnedEnforcedConstraint::new(
            constraint,
            Method::new(
                method.to_string(),
                inputs.to_vec(),
                outputs.to_vec(),
                Arc::new(Ok),
            ),
        )
    }

    #[test]
    fn not_adjusting_priorities_gives_unexpected_result() {
        let component = crate::examples::components::priority_adjust::priority_adjust();
        // Create ranking
        let mut ranker = SortRanker::of_size(component.n_variables());
        ranker.touch(0);
        ranker.touch(3);
        ranker.touch(2);
        let ranking = ranker.ranking();
        assert_eq!(ranking, vec![2, 3, 0, 1]);

        // See what happens if we don't update the ranking
        ranker.touch(2);
        let plan = hierarchical_planner(&component, &ranker.ranking()).unwrap();
        assert_eq!(
            plan,
            vec![
                dummy("Ab", "m1", &[0], &[1]),
                dummy("Bcd", "m3", &[1, 2], &[3]),
            ]
        );
    }

    #[test]
    fn adjusting_priorities_gives_expected_result() {
        let component = crate::examples::components::priority_adjust::priority_adjust();
        // Create ranking
        let mut ranker = SortRanker::of_size(component.n_variables());
        ranker.touch(0);
        ranker.touch(3);
        let ranking = ranker.ranking();
        assert_eq!(ranking, vec![3, 0, 1, 2]);

        // See that the expected plan is found
        let plan = hierarchical_planner(&component, &ranking).unwrap();
        assert_eq!(
            plan,
            vec![
                dummy("Bcd", "m4", &[3], &[1, 2]),
                dummy("Ab", "m2", &[1], &[0]),
            ]
        );

        // Update the ranking based on the plan
        let mut new_ranker = adjust_priorities(&plan, &ranker);
        assert_eq!(new_ranker.ranking(), &[3, 1, 0, 2]);

        // Verify that the updated ranking works as expected
        new_ranker.touch(2);
        let plan = hierarchical_planner(&component, &new_ranker.ranking()).unwrap();
        assert_eq!(
            plan,
            vec![
                dummy("Bcd", "m3", &[1, 2], &[3]),
                dummy("Ab", "m2", &[1], &[0]),
            ]
        );
    }

    #[test]
    fn adjusting_priorities_in_sum_gives_expected_result() {
        let component = crate::examples::components::numbers::sum::<i32>();
        // Create ranking
        let mut ranker = SortRanker::of_size(component.n_variables());
        ranker.touch(0);
        let ranking = ranker.ranking();
        assert_eq!(ranking, vec![0, 1, 2]);

        // See that the expected plan is found
        let plan = hierarchical_planner(&component, &ranking).unwrap();
        assert_eq!(plan, vec![dummy("Sum", "abc", &[0, 1], &[2])]);

        // Update the ranking based on the plan
        let mut new_ranker = adjust_priorities(&plan, &ranker);
        assert_eq!(new_ranker.ranking(), &[0, 1, 2]);

        // Verify that the updated ranking works as expected
        new_ranker.touch(2);
        let plan = hierarchical_planner(&component, &new_ranker.ranking()).unwrap();
        assert_eq!(plan, vec![dummy("Sum", "acb", &[0, 2], &[1]),]);
    }
}
