#[allow(dead_code)]
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<i32> = nums.iter().map(|x| x * x).collect();
        nums.sort();
        nums
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![-4, -1, 0, 3, 10];

        let expected_result = vec![0, 1, 9, 16, 100];
        let actual_result = Solution::sorted_squares(nums);

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn example_2() {
        let nums = vec![-7, -3, 2, 3, 11];

        let expected_result = vec![4, 9, 9, 49, 121];
        let actual_result = Solution::sorted_squares(nums);

        assert_eq!(expected_result, actual_result)
    }
}

struct Solution {}
