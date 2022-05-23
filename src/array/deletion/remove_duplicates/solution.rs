#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut shift_amount = 0;
        let mut i = 1;

        loop {
            if i >= nums.len() {
                break;
            }

            if nums[i] == nums[i - 1] {
                shift_amount += 1;
            } else {
                nums[i - shift_amount] = nums[i];
            }

            i += 1;
        }

        (nums.len() - shift_amount) as i32
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 2];

        let expected_output = 2;
        let expected_array = vec![1, 2];

        let actual_output = Solution::remove_duplicates(&mut nums);

        assert_eq!(actual_output, expected_output);
        assert!(nums.starts_with(&expected_array))
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        let expected_output = 5;
        let expected_array = vec![0, 1, 2, 3, 4];

        let actual_output = Solution::remove_duplicates(&mut nums);

        assert_eq!(actual_output, expected_output);
        assert!(nums.starts_with(&expected_array))
    }
}
