#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut deletion_count = 0;

        for i in 0..nums.len() {
            if nums[i] == val {
                deletion_count += 1;
            } else {
                nums[i - deletion_count] = nums[i];
            }
        }

        (nums.len() - deletion_count) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;

        let expected_k = 2;
        let actual_k = Solution::remove_element(&mut nums, val);

        let expected_nums = vec![2, 2];

        assert_eq!(expected_k, actual_k);
        assert!(nums.starts_with(&expected_nums));
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;

        let expected_k = 5;
        let actual_k = Solution::remove_element(&mut nums, val);

        let expected_nums = vec![0, 1, 3, 0, 4];

        assert_eq!(expected_k, actual_k);
        assert!(nums.starts_with(&expected_nums));
    }
}

struct Solution {}
