use super::variable_ranker::VariableRanker;

/// A simple ranker that keeps a counter that is incremented
/// when a variable is updated. Store the latest counter value
/// for each variable, and rank them based on their counter values.
#[derive(Clone, PartialEq, Debug)]
pub struct SortRanker {
    counter: usize,
    data: Vec<usize>,
}

impl VariableRanker for SortRanker {
    fn of_size(size: usize) -> Self {
        Self {
            counter: 0,
            data: vec![0; size],
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    /// Update the ranking in O(1) time
    fn touch(&mut self, index: usize) {
        self.counter += 1;
        self.data[index] = self.counter;
    }

    /// Generate the ranking in O(n * log(n)) time
    fn ranking(&self) -> Vec<usize> {
        let mut values: Vec<usize> = (0..self.data.len()).collect();
        values.sort_by(|&i, &j| self.data[j].cmp(&self.data[i]));
        values
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::{SortRanker, VariableRanker};

    #[test]
    fn ranking_zero_vars_should_give_empty_vec() {
        let vr = SortRanker::of_size(0);
        let expected: Vec<usize> = Vec::new();
        assert_eq!(vr.ranking(), expected);
    }

    #[test]
    fn ranking_one_var_should_give_singleton_vec() {
        let mut vr = SortRanker::of_size(1);
        vr.touch(0);
        assert_eq!(vr.ranking(), vec![0]);
    }

    #[test]
    fn ranking_three_vars_after_touching_first_should_have_first_at_head() {
        let mut vr = SortRanker::of_size(3);
        vr.touch(0);
        assert_eq!(vr.ranking(), vec![0, 1, 2]);
    }

    #[test]
    fn ranking_three_vars_after_touching_second_should_have_second_at_head() {
        let mut vr = SortRanker::of_size(3);
        vr.touch(1);
        assert_eq!(vr.ranking(), vec![1, 0, 2]);
    }

    #[test]
    fn ranking_three_vars_after_touching_third_should_have_third_at_head() {
        let mut vr = SortRanker::of_size(3);
        vr.touch(2);
        assert_eq!(vr.ranking(), vec![2, 0, 1]);
    }

    #[test]
    fn rank_more_variables_1() {
        let mut vr = SortRanker::of_size(5);
        let mut expected = vec![3, 4, 2, 0, 1];
        for &i in &expected {
            vr.touch(i);
        }
        expected.reverse();
        assert_eq!(vr.ranking(), expected);
    }

    #[test]
    fn rank_more_variables_2() {
        let n_variables = 10;
        let mut vr = SortRanker::of_size(n_variables);
        let mut expected: Vec<usize> = (0..n_variables).collect();
        for &i in &expected {
            vr.touch(i);
        }
        expected.reverse();
        assert_eq!(vr.ranking(), expected);
    }

    #[bench]
    fn touch_variables_bench(b: &mut test::Bencher) {
        let n_variables = 50000;
        let mut vr = SortRanker::of_size(n_variables);
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
        let vr = SortRanker::of_size(n_variables);
        let mut expected: Vec<usize> = (0..n_variables).collect();
        expected.reverse();
        b.iter(|| vr.ranking())
    }

    #[bench]
    fn touch_and_rank_variables_bench(b: &mut test::Bencher) {
        let n_variables = 100000;
        let mut vr = SortRanker::of_size(n_variables);
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
