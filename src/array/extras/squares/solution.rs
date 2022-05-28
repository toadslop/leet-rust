#[allow(dead_code)]
impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = nums.len() - 1;
        println!("{:?}", nums);
        println!("");
        while i <= j {
            println!("{} {}", nums[i], nums[j]);
            if nums[i].abs() > nums[j].abs() {
                nums.swap(i, j);
                nums[j] = nums[j].pow(2);
                j -= 1;
            } else {
                if j > 0 && nums[j].abs() >= nums[j - 1].abs() {
                    nums[j] = nums[j].pow(2);
                    j -= 1;
                } else {
                    nums[i] = nums[i].pow(2);
                    i += 1;
                }
            }

            println!("{:?}", nums);
            println!("");
        }
        nums
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    // use super::Solution;

    // #[test]
    // fn example_1() {
    //     let nums = vec![-4, -1, 0, 3, 10];

    //     let expected_output = vec![0, 1, 9, 16, 100];
    //     let actual_output = Solution::sorted_squares(nums);

    //     assert_eq!(expected_output, actual_output)
    // }

    // #[test]
    // fn example_2() {
    //     let nums = vec![-7, -3, 2, 3, 11];

    //     let expected_output = vec![4, 9, 9, 49, 121];
    //     let actual_output = Solution::sorted_squares(nums);

    //     assert_eq!(expected_output, actual_output)
    // }

    // #[test]
    // fn example_3() {
    //     let nums = vec![-5, -3, -2, -1];

    //     let expected_output = vec![1, 4, 9, 25];
    //     let actual_output = Solution::sorted_squares(nums);

    //     assert_eq!(expected_output, actual_output)
    // }

    // #[test]
    // fn example_4() {
    //     let nums = vec![-5, -3, -2, -1, 1];

    //     let expected_output = vec![1, 1, 4, 9, 25];
    //     let actual_output = Solution::sorted_squares(nums);

    //     assert_eq!(expected_output, actual_output)
    // }

    // #[test]
    // fn example_5() {
    //     let nums = vec![-5, -3, -2, -1, 1, 4];

    //     let expected_output = vec![1, 1, 4, 9, 16, 25];
    //     let actual_output = Solution::sorted_squares(nums);

    //     assert_eq!(expected_output, actual_output)
    // }

    // #[test]
    // fn example_6() {
    //     let nums = vec![-5, -3, -2, -1, 1, 4, 5];

    //     let expected_output = vec![1, 1, 4, 9, 16, 25, 25];
    //     let actual_output = Solution::sorted_squares(nums);

    //     assert_eq!(expected_output, actual_output)
    // }

    // #[test]
    // fn example_7() {
    //     let nums = vec![-5, -3, 2, 1, 4, 5];

    //     let expected_output = vec![1, 4, 9, 16, 25, 25];
    //     let actual_output = Solution::sorted_squares(nums);

    //     assert_eq!(expected_output, actual_output)
    // }

    // #[test]
    // fn example_8() {
    //     let nums = vec![-1, 0, 1, 1];

    //     let expected_output = vec![0, 1, 1, 1];
    //     let actual_output = Solution::sorted_squares(nums);

    //     assert_eq!(expected_output, actual_output)
    // }

    // #[test]
    // fn example_9() {
    //     let nums = vec![-5, -4, 1, 2, 5];

    //     let expected_output = vec![1, 4, 16, 25, 25];
    //     let actual_output = Solution::sorted_squares(nums);

    //     assert_eq!(expected_output, actual_output)
    // }
}
