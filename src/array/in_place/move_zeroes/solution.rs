#[allow(dead_code)]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }

        let mut zero_count = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[i - zero_count] = nums[i];
            } else {
                zero_count += 1;
            }

            if zero_count > 0 {
                nums[i] = 0;
            }
        }
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;


    #[test]
    fn example_1() {
        let mut arr = vec![0,1,0,3,12];

        let expected_output = vec![1,3,12,0,0];
        Solution::move_zeroes(&mut arr);

        assert_eq!(expected_output, arr)
    }

    #[test]
    fn example_2() {
        let mut arr = vec![0];

        let expected_output = vec![0];
        Solution::move_zeroes(&mut arr);

        assert_eq!(expected_output, arr)
    }

    #[test]
    fn example_3() {
        let mut arr = vec![0,1,0,3,0];

        let expected_output = vec![1,3,0,0,0];
        Solution::move_zeroes(&mut arr);

        assert_eq!(expected_output, arr)
    }

    #[test]
    fn example_4() {
        let mut arr = vec![-58305,-22022,0,0,0,0,0,-76070,42820,48119,0,95708,-91393,60044,0,-34562,0,-88974];

        let expected_output = vec![-58305,-22022,-76070,42820,48119,95708,-91393,60044,-34562,-88974,0,0,0,0,0,0,0,0];
        Solution::move_zeroes(&mut arr);

        assert_eq!(expected_output, arr)
    }

    #[test]
    fn example_5() {
        let mut arr = vec![2,1];

        let expected_output = vec![2,1];
        Solution::move_zeroes(&mut arr);

        assert_eq!(expected_output, arr)
    }

    #[test]
    fn example_6() {
        let mut arr = vec![-13009,0,-81471,93346,0,-71602,-18829,-45703,0,0,0,15246,0,51324,89825,0,70362,0,50913,0,47988,-87456,94441,0,0,77733,9338];

        let expected_output = vec![-13009,-81471,93346,-71602,-18829,-45703,15246,51324,89825,70362,50913,47988,-87456,94441,77733,9338,0,0,0,0,0,0,0,0,0,0,0];
        Solution::move_zeroes(&mut arr);

        assert_eq!(expected_output, arr)
    }
}