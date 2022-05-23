use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut doubles = HashSet::new();

        for num in arr.iter() {
            if doubles.contains(num) {
                return true;
            }

            doubles.insert(num * 2);

            if num % 2 == 0 {
                doubles.insert(num / 2);
            }
        }

        false
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let arr = vec![10, 2, 5, 3];

        let expected_output = true;
        let actual_output = Solution::check_if_exist(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_2() {
        let arr = vec![7, 1, 14, 11];

        let expected_output = true;
        let actual_output = Solution::check_if_exist(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_3() {
        let arr = vec![3, 1, 7, 11];

        let expected_output = false;
        let actual_output = Solution::check_if_exist(arr);

        assert_eq!(actual_output, expected_output);
    }
}
