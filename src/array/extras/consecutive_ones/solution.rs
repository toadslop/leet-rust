#[allow(dead_code)]
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ones_since_last_zero: u16 = 0;
        let mut ones_before_last_zero: u16 = 0;
        let mut zero_mod: u16 = 0;
        let mut max: u16 = 0;

        for num in nums.iter() {
            if *num == 1 {
                ones_since_last_zero += 1;
            } else {
                zero_mod = 1;
                ones_before_last_zero = ones_since_last_zero;
                ones_since_last_zero = 0;
            }

            if ones_since_last_zero + ones_before_last_zero + zero_mod > max {
                max = ones_since_last_zero + ones_before_last_zero + zero_mod;
            }
        }

        max as i32
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![1, 0, 1, 1, 0];

        let expected_output = 4;
        let actual_output = Solution::find_max_consecutive_ones(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 0, 1, 1, 0, 1];

        let expected_output = 4;
        let actual_output = Solution::find_max_consecutive_ones(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_3() {
        let nums = vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0];

        let expected_output = 7;
        let actual_output = Solution::find_max_consecutive_ones(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_4() {
        let nums = vec![1, 1, 0, 1];

        let expected_output = 4;
        let actual_output = Solution::find_max_consecutive_ones(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_5() {
        let nums = vec![1];

        let expected_output = 1;
        let actual_output = Solution::find_max_consecutive_ones(nums);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_6() {
        let nums = vec![0];

        let expected_output = 1;
        let actual_output = Solution::find_max_consecutive_ones(nums);

        assert_eq!(expected_output, actual_output)
    }
}
