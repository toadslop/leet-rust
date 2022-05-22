#[allow(dead_code)]
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut j = m - 1;
        let mut k = n - 1;

        for i in (0..(m + n) as usize).rev() {
            let num1 = nums1.get(j as usize).unwrap_or(&i32::MIN);
            let num2 = nums2.get(k as usize).unwrap_or(&i32::MIN);
            if num2 > num1 {
                nums1[i] = *num2;
                k -= 1;
            } else {
                nums1[i] = *num1;
                j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        let expected_result = vec![1, 2, 2, 3, 5, 6];
        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, expected_result)
    }

    #[test]
    fn example_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        let expected_result = vec![1];
        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, expected_result)
    }

    #[test]
    fn example_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        let expected_result = vec![1];
        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, expected_result)
    }
}

struct Solution {}
