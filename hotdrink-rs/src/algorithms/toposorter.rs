//! A function for topologically sorting plans.
//! This must be done before executing them.

use super::hierarchical_planner::Vertex;

/// Topologically sort a slice of method references.
pub fn toposort<M>(methods: &[M], n_variables: usize) -> Option<Vec<&M>>
where
    M: Vertex,
{
    let n_methods = methods.len();
    // Create adjacency list to go from a variable to methods using it as input
    let mut var_to_methods = vec![Vec::new(); n_variables];
    for (m_id, m) in methods.iter().enumerate() {
        // Variable -> methods
        for &var_id in m.inputs() {
            var_to_methods[var_id].push(m_id);
        }
    }

    // Create an adjacency list to represent the method graph
    let mut method_to_methods = vec![Vec::new(); methods.len()];
    // For each method and its id
    for (m_id, m) in methods.iter().enumerate() {
        // For each of its outputs
        for &out_var in m.outputs() {
            // Add an edge to methods using this output as input
            for &m_target in &var_to_methods[out_var] {
                // But ignore self-loops
                if m_target != m_id {
                    method_to_methods[m_id].push(m_target);
                }
            }
        }
    }

    // Set all methods to not marked
    let mut marked = vec![false; n_methods];
    let mut visiting = vec![false; n_methods];
    let mut order = Vec::new();
    for start in 0..n_methods {
        if !marked[start] {
            dfs(
                start,
                &method_to_methods,
                &mut marked,
                &mut visiting,
                &mut order,
            )?;
        }
    }

    Some(order.into_iter().map(|m_id| &methods[m_id]).rev().collect())
}

/// Dfs through an adjacency list and store post-numbers.
fn dfs(
    start: usize,
    adjacency_list: &[Vec<usize>],
    marked: &mut Vec<bool>,
    visiting: &mut Vec<bool>,
    order: &mut Vec<usize>,
) -> Option<()> {
    // Loop detected, we are already visiting this node
    if visiting[start] {
        return None;
    }

    // Reached part of graph that is already done, just return
    if marked[start] {
        return Some(());
    }

    marked[start] = true;
    visiting[start] = true;
    let next_methods = &adjacency_list[start];
    // For methods that take the variable as input
    for &m in next_methods {
        // Recurse to the next method
        dfs(m, adjacency_list, marked, visiting, order)?;
    }

    // We are done with `start`, push it and leave
    order.push(start);
    visiting[start] = false;

    Some(())
}

#[cfg(test)]
mod tests {

    use super::toposort;
    use crate::algorithms::hierarchical_planner::Vertex;

    /// A dummy method for testing
    #[derive(PartialEq, Eq, Debug, Clone)]
    struct DummyMethod {
        inputs: Vec<usize>,
        outputs: Vec<usize>,
    }

    impl DummyMethod {
        fn new(inputs: Vec<usize>, outputs: Vec<usize>) -> Self {
            Self { inputs, outputs }
        }
        fn dummy(inputs: Vec<usize>, outputs: Vec<usize>) -> Self {
            Self { inputs, outputs }
        }
    }

    impl Vertex for DummyMethod {
        fn inputs(&self) -> &[usize] {
            &self.inputs
        }

        fn outputs(&self) -> &[usize] {
            &self.outputs
        }

        fn stay(index: usize) -> Self {
            Self::new(vec![index], vec![index])
        }

        fn is_stay(&self) -> bool {
            false
        }
    }

    #[test]
    fn empty_plan_gives_empty_sorted_plan() {
        assert_eq!(
            toposort::<DummyMethod>(&[], 0),
            Some(Vec::new()),
            "Empty plan should yield empty sorted plan"
        );
    }

    #[test]
    fn singleton_plan_gives_singleton_sorted_plan() {
        let m0 = DummyMethod::dummy(vec![0], vec![1]);
        assert_eq!(
            toposort(&[m0.clone()], 2),
            Some(vec![&m0]),
            "m0 should be included"
        );
    }

    #[test]
    fn single_method_with_loop_is_ok() {
        let m0 = DummyMethod::dummy(vec![0], vec![0]);
        assert_eq!(toposort(&[m0.clone()], 1), Some(vec![&m0]));
    }

    #[test]
    fn a_to_b_is_ok() {
        let m0 = DummyMethod::dummy(vec![0], vec![1]);
        let m1 = DummyMethod::dummy(vec![1], vec![2]);
        assert_eq!(toposort(&[m0.clone(), m1.clone()], 3), Some(vec![&m0, &m1]));
        assert_eq!(toposort(&[m1.clone(), m0.clone()], 3), Some(vec![&m0, &m1]));
    }

    #[test]
    fn cycle_gives_none() {
        let m0 = DummyMethod::dummy(vec![0], vec![1]);
        let m1 = DummyMethod::dummy(vec![1], vec![0]);
        assert_eq!(toposort(&[m0.clone(), m1.clone()], 2), None);
        assert_eq!(toposort(&[m1, m0], 2), None);
    }

    #[test]
    fn larger_example_is_ok() {
        let m0 = DummyMethod::dummy(vec![0], vec![1, 2]);
        let m1 = DummyMethod::dummy(vec![1], vec![3]);
        let m2 = DummyMethod::dummy(vec![2], vec![4, 5]);
        let m3 = DummyMethod::dummy(vec![5, 6], vec![7]);
        assert_eq!(
            toposort(&[m0.clone(), m1.clone(), m2.clone(), m3.clone()], 8),
            Some(vec![&m0, &m2, &m3, &m1])
        );
    }
}
