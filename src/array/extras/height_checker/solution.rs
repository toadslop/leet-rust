#[allow(dead_code)]
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut correct = heights.clone();
        correct.sort();

        correct
            .iter()
            .enumerate()
            .fold(0, |mut total_incorrect, (i, current)| {
                if *current != heights[i] {
                    total_incorrect += 1
                };
                total_incorrect
            })
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let heights = vec![1, 1, 4, 2, 1, 3];

        let expected_output = 3;
        let actual_output = Solution::height_checker(heights);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_2() {
        let heights = vec![5, 1, 2, 3, 4];

        let expected_output = 5;
        let actual_output = Solution::height_checker(heights);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_3() {
        let heights = vec![1, 2, 3, 4, 5];

        let expected_output = 0;
        let actual_output = Solution::height_checker(heights);

        assert_eq!(expected_output, actual_output)
    }
}
