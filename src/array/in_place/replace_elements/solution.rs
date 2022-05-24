#[allow(dead_code)]
impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut ge = -1;
        
        for i in (0..arr.len()).rev() {
            let temp = arr[i];
            arr[i] = ge;

            if temp > ge {
                ge = temp;
            }
        }

        arr
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;


    #[test]
    fn example_1() {
        let arr = vec![17,18,5,4,6,1];

        let expected_output = vec![18,6,6,6,1,-1];
        let actual_output = Solution::replace_elements(arr);

        assert_eq!(expected_output, actual_output)
    }

    #[test]
    fn example_2() {
        let arr = vec![400];

        let expected_output = vec![-1];
        let actual_output = Solution::replace_elements(arr);

        assert_eq!(expected_output, actual_output)
    }
}