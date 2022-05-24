#[allow(dead_code)]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        unimplemented!()
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use super::Solution;


    #[test]
    fn example_1() {
        let mut arr = vec![17,18,5,4,6,1];

        let expected_output = vec![0,1,0,3,12];
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
}