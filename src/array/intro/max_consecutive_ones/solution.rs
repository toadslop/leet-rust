use std::cmp::max;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_ones = 0;
        let mut current_count = 0;

        for num in nums.iter() {
            if *num == 1 {
                current_count += 1;
            } else {
                max_ones = max(current_count, max_ones);
                current_count = 0;
            }
        }

        max(current_count, max_ones)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = Vec::from([1, 1, 0, 1, 1, 1]);
        let expected_result = 3;
        let actual_result = Solution::find_max_consecutive_ones(nums);

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn example_2() {
        let nums = Vec::from([1, 0, 1, 1, 0, 1]);
        let expected_result = 2;
        let actual_result = Solution::find_max_consecutive_ones(nums);

        assert_eq!(expected_result, actual_result)
    }
}

struct Solution {}
