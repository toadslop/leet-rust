#[allow(dead_code)]
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for num in nums.iter() {
            if calc_num_digits(*num) % 2 == 0 {
                count += 1;
            }
        }
        count
    }
}

fn calc_num_digits(mut num: i32) -> i32 {
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums = vec![12, 345, 2, 6, 7896];

        let expected_result = 2;
        let actual_result = Solution::find_numbers(nums);

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn example_2() {
        let nums = vec![555, 901, 482, 1771];

        let expected_result = 1;
        let actual_result = Solution::find_numbers(nums);

        assert_eq!(expected_result, actual_result)
    }
}

struct Solution {}
