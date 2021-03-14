//! A faster variable ranker.
//!
//! This one performs ranking updates in constant time,
//! and then only requires linear time to gather the
//! results in a vector.

use super::variable_ranker::VariableRanker;
use std::iter::{self, Iterator};

/// Keeps track of the root (if one exists),
/// and two adjacency lists specifying the previous
/// and next node in the ranking order.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LinkedListRanker {
    size: usize,
    root: Option<usize>,
    prev_of: Vec<Option<usize>>,
    next_of: Vec<Option<usize>>,
}

impl VariableRanker for LinkedListRanker {
    fn of_size(size: usize) -> Self {
        if size == 0 {
            LinkedListRanker {
                size,
                root: None,
                prev_of: Vec::new(),
                next_of: Vec::new(),
            }
        } else {
            LinkedListRanker {
                size,
                root: Some(0),
                prev_of: iter::once(None).chain((0..size - 1).map(Some)).collect(),
                next_of: (1..size).map(Some).chain(iter::once(None)).collect(),
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    /// Update the ranking in O(1) time
    fn touch(&mut self, current: usize) {
        if current >= self.size {
            log::error!(
                "Touched variable {}, which is outside of size {}",
                current,
                self.size
            );
            return;
        }

        // Nothing to do
        if self.root == Some(current) {
            return;
        }

        // A. Glue prev and next together

        // 1. Point prev to next
        if let Some(prev) = self.prev_of[current] {
            self.next_of[prev] = self.next_of[current];
        }

        // 2. Point next to prev
        if let Some(next) = self.next_of[current] {
            self.prev_of[next] = self.prev_of[current];
        }

        // B. Swap out the root

        // 1. Point current to old root
        self.next_of[current] = self.root;

        // 2. Make old root point to current
        if let Some(root) = self.root {
            self.prev_of[root] = Some(current);
        }

        // 3. Set current to new root
        self.root = Some(current);
    }

    /// Generate the ranking in O(n) time
    fn ranking(&self) -> Vec<usize> {
        self.iter().collect()
    }
}

impl LinkedListRanker {
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        FastRankerIterator {
            ranker: &self,
            current: self.root,
            done: false,
        }
    }
}

/// An iterator for iterating through
/// the ranking of a `FastRanker` without
/// having to collect it all in a vector.
#[derive(Debug)]
struct FastRankerIterator<'a> {
    ranker: &'a LinkedListRanker,
    current: Option<usize>,
    done: bool,
}

impl<'a> Iterator for FastRankerIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // Save current
        let current = self.current;
        // Update current if the end has not been reached
        if let Some(current) = current {
            self.current = self.ranker.next_of[current];
        }
        current
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn correct_construction_0() {
        let fr = LinkedListRanker::of_size(0);
        assert_eq!(
            fr,
            LinkedListRanker {
                size: 0,
                root: None,
                prev_of: vec![],
                next_of: vec![]
            }
        )
    }
    #[test]
    fn correct_construction_1() {
        let fr = LinkedListRanker::of_size(1);
        assert_eq!(
            fr,
            LinkedListRanker {
                size: 1,
                root: Some(0),
                prev_of: vec![None],
                next_of: vec![None]
            }
        )
    }

    #[test]
    fn correct_construction_2() {
        let fr = LinkedListRanker::of_size(2);
        assert_eq!(
            fr,
            LinkedListRanker {
                size: 2,
                root: Some(0),
                prev_of: vec![None, Some(0)],
                next_of: vec![Some(1), None]
            }
        )
    }
    #[test]
    fn correct_construction_3() {
        let fr = LinkedListRanker::of_size(3);
        assert_eq!(
            fr,
            LinkedListRanker {
                size: 3,
                root: Some(0),
                prev_of: vec![None, Some(0), Some(1)],
                next_of: vec![Some(1), Some(2), None]
            }
        )
    }

    #[test]
    fn trivial_rank() {
        let fr = LinkedListRanker::of_size(3);
        assert_eq!(fr.ranking(), vec![0, 1, 2])
    }

    #[test]
    fn expected_ranking_2() {
        let mut fr = LinkedListRanker::of_size(5);
        fr.touch(3);
        fr.touch(2);
        fr.touch(4);
        fr.touch(1);
        fr.touch(0);
        fr.touch(4);
        fr.touch(2);
        fr.touch(3);
        assert_eq!(fr.ranking(), vec![3, 2, 4, 0, 1])
    }

    #[test]
    fn touch_in_order_is_ok() {
        let mut fr = LinkedListRanker::of_size(3);
        fr.touch(0);
        fr.touch(1);
        fr.touch(2);
        assert_eq!(fr.ranking(), vec![2, 1, 0])
    }

    #[test]
    fn touch_root_is_ok() {
        let mut fr = LinkedListRanker::of_size(1);
        fr.touch(0);
        fr.touch(0);
        assert_eq!(fr.ranking(), vec![0]);
    }

    #[test]
    fn ranking_three_vars_after_touching_first_should_have_first_at_head() {
        let mut vr = LinkedListRanker::of_size(3);
        vr.touch(0);
        assert_eq!(vr.ranking(), vec![0, 1, 2]);
    }

    #[test]
    fn ranking_three_vars_after_touching_second_should_have_second_at_head() {
        let mut vr = LinkedListRanker::of_size(3);
        vr.touch(1);
        assert_eq!(vr.ranking(), vec![1, 0, 2]);
    }

    #[test]
    fn ranking_three_vars_after_touching_third_should_have_third_at_head() {
        let mut vr = LinkedListRanker::of_size(3);
        vr.touch(2);
        assert_eq!(vr.ranking(), vec![2, 0, 1]);
    }

    #[bench]
    fn touch_variables_bench(b: &mut test::Bencher) {
        let n_variables = 50000;
        let mut vr = LinkedListRanker::of_size(n_variables);
        let mut expected: Vec<usize> = (0..n_variables).collect();
        expected.reverse();
        b.iter(|| {
            for &i in &expected {
                vr.touch(i);
            }
        })
    }

    #[bench]
    fn rank_variables_bench(b: &mut test::Bencher) {
        let n_variables = 100000;
        let vr = LinkedListRanker::of_size(n_variables);
        let mut expected: Vec<usize> = (0..n_variables).collect();
        expected.reverse();
        b.iter(|| vr.ranking())
    }

    #[bench]
    fn touch_and_rank_variables_bench(b: &mut test::Bencher) {
        let n_variables = 100000;
        let mut vr = LinkedListRanker::of_size(n_variables);
        let mut expected: Vec<usize> = (0..n_variables).collect();
        expected.reverse();
        b.iter(|| {
            for &i in &expected {
                vr.touch(i);
            }
            vr.ranking()
        })
    }
}
