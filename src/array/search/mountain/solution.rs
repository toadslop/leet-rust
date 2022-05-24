#[allow(dead_code)]
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }

        if arr[1] < arr[0] {
            return false;
        }

        let mut reached_peak = false;

        for i in 1..arr.len() - 1 {
           if arr[i] > arr[i - 1] && arr[i] > arr[i + 1] {
               reached_peak = true;
           }

           if arr[i] == arr[i - 1] || arr[i] == arr[i + 1] {
               return false;
           }

           if reached_peak && arr[i + 1] > arr[i] {
               return false;
           }
              
        }

        true && reached_peak
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use crate::array::search::mountain::solution::Solution;

    #[test]
    fn example_1() {
        let arr = vec![2,1];

        let expected_output = false;
        let actual_output = Solution::valid_mountain_array(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_2() {
        let arr = vec![3,5,5];

        let expected_output = false;
        let actual_output = Solution::valid_mountain_array(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_3() {
        let arr = vec![0,3,2,1];

        let expected_output = true;
        let actual_output = Solution::valid_mountain_array(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_4() {
        let arr = vec![0,3,4,5];

        let expected_output = false;
        let actual_output = Solution::valid_mountain_array(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_5() {
        let arr = vec![5,4,3,2];

        let expected_output = false;
        let actual_output = Solution::valid_mountain_array(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_6() {
        let arr = vec![1,5,4,3,2,3];

        let expected_output = false;
        let actual_output = Solution::valid_mountain_array(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_7() {
        let arr = vec![2,1,5,4,3,2,1];

        let expected_output = false;
        let actual_output = Solution::valid_mountain_array(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_8() {
        let arr = vec![1,1,1,1,1,1,1,2,1];

        let expected_output = false;
        let actual_output = Solution::valid_mountain_array(arr);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn example_9() {
        let arr = vec![1,2,1,1,1,1,1,1,1];

        let expected_output = false;
        let actual_output = Solution::valid_mountain_array(arr);

        assert_eq!(actual_output, expected_output);
    }

    
}
