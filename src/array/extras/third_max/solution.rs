use std::cmp::Reverse;

#[allow(dead_code)]
impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| Reverse(a).cmp(&Reverse(b)));
        nums.dedup();

        if let Some(third) = nums.get(2) {
            *third
        } else {
            nums[0]
        }
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3, 2, 1];

        let expected_output = 1;
        let actual_output = Solution::third_max(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2];

        let expected_output = 2;
        let actual_output = Solution::third_max(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_3() {
        let nums = vec![2, 2, 3, 1];

        let expected_output = 1;
        let actual_output = Solution::third_max(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_4() {
        let nums = vec![2, 2, 3, 1, 5, 2];

        let expected_output = 2;
        let actual_output = Solution::third_max(nums);

        assert_eq!(expected_output, actual_output)
    }
}
