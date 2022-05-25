#[allow(dead_code)]
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut last_odd = 0;

        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                nums.swap(i, last_odd);
                last_odd += 1;
            }
        }

        nums
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![3,1,2,4];

        let expected_outputs: Vec<Vec<i32>> = Vec::from([[2,4,3,1], [4,2,3,1], [2,4,1,3],  [4,2,1,3]]).iter().map(|arr| Vec::from(*arr)).collect();
        let actual_output = Solution::sort_array_by_parity(nums);

        assert!(expected_outputs.contains(&actual_output))
    }

    #[test]
    fn example_2() {
        let nums = vec![0];

        let expected_output = vec![0];
        let actual_output = Solution::sort_array_by_parity(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_3() {
        let nums = vec![3,2,1,4];

        let expected_outputs: Vec<Vec<i32>> = Vec::from([[2,4,3,1], [4,2,3,1], [2,4,1,3],  [4,2,1,3]]).iter().map(|arr| Vec::from(*arr)).collect();
        let actual_output = Solution::sort_array_by_parity(nums);

        assert!(expected_outputs.contains(&actual_output))
    }

    #[test]
    fn example_4() {
        let nums = vec![2,4,3,1];

        let expected_outputs: Vec<Vec<i32>> = Vec::from([[2,4,3,1], [4,2,3,1], [2,4,1,3],  [4,2,1,3]]).iter().map(|arr| Vec::from(*arr)).collect();
        let actual_output = Solution::sort_array_by_parity(nums);

        assert!(expected_outputs.contains(&actual_output))
    }
}