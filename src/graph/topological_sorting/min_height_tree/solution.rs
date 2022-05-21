impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 4;
        let edges: Vec<Vec<i32>> = Vec::from([[1, 0], [1, 2], [1, 3]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let mut expected_result = Vec::from([1]);
        expected_result.sort();

        let mut actual_result = Solution::find_min_height_trees(n, edges);
        actual_result.sort();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_2() {
        let n = 6;
        let edges: Vec<Vec<i32>> = Vec::from([[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let mut expected_result = Vec::from([1]);
        expected_result.sort();
        let mut actual_result = Solution::find_min_height_trees(n, edges);
        actual_result.sort();

        assert_eq!(expected_result, actual_result);
    }
}

#[allow(dead_code)]
struct Solution {}
