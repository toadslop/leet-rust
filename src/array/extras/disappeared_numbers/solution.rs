use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::<i32>::from_iter(1..=nums.len() as i32);
        for num in nums.iter() {
            set.remove(num);
        }

        set.into_iter().collect()
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];

        let expected_output = vec![5, 6];
        let actual_output = Solution::find_disappeared_numbers(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 1];

        let expected_output = vec![2];
        let actual_output = Solution::find_disappeared_numbers(nums);

        assert_eq!(expected_output, actual_output)
    }
}
