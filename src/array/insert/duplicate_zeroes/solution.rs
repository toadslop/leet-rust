#[allow(dead_code)]
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut skip = false;
        for i in 1..arr.len() {
            if arr[i - 1] == 0 && !skip {
                let mut temp = arr[i];
                arr[i] = 0;

                for j in i + 1..arr.len() {
                    let temp2 = arr[j];
                    arr[j] = temp;
                    temp = temp2;
                }
                skip = true;
            } else {
                skip = false;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];

        let expected_result = vec![1, 0, 0, 2, 3, 0, 0, 4];
        Solution::duplicate_zeros(&mut arr);

        assert_eq!(expected_result, arr)
    }

    #[test]
    fn example_2() {
        let mut arr = vec![1, 2, 3];

        let expected_result = vec![1, 2, 3];
        Solution::duplicate_zeros(&mut arr);

        assert_eq!(expected_result, arr)
    }
}

struct Solution {}
