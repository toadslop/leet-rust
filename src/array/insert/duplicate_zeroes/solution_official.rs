#[allow(dead_code)]
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut possible_dups = 0;
        let mut length = arr.len() - 1;

        for left in 0..arr.len() {
            if left >= length - possible_dups {
                println!("BROKE");
                break;
            }

            if arr[left] == 0 {
                println!(
                    "LEN {}, DUPS COUNT {}, LEFT {}",
                    length, possible_dups, left
                );
                if left == length - possible_dups - 1 {
                    arr[length] = 0;
                    length -= 1;
                    break;
                }

                possible_dups += 1;
            }
        }

        for current in (0..length - possible_dups).rev() {
            if arr[current] == 0 {
                arr[current + possible_dups] = 0;
                possible_dups -= 1;
                arr[current + possible_dups] = 0;
            } else {
                arr[current + possible_dups] = arr[current]
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

    #[test]
    fn example_3() {
        let mut arr = vec![8, 4, 5, 0, 0, 0, 0, 7];

        let expected_result = vec![8, 4, 5, 0, 0, 0, 0, 0];
        Solution::duplicate_zeros(&mut arr);

        assert_eq!(expected_result, arr)
    }

    #[test]
    fn example_4() {
        let mut arr = vec![
            9, 9, 9, 4, 8, 0, 0, 3, 7, 2, 0, 0, 0, 0, 9, 1, 0, 0, 1, 1, 0, 5, 6, 3, 1, 6, 0, 0, 2,
            3, 4, 7, 0, 3, 9, 3, 6, 5, 8, 9, 1, 1, 3, 2, 0, 0, 7, 3, 3, 0, 5, 7, 0, 8, 1, 9, 6, 3,
            0, 8, 8, 8, 8, 0, 0, 5, 0, 0, 0, 3, 7, 7, 7, 7, 5, 1, 0, 0, 8, 0, 0,
        ];

        let expected_result = vec![
            9, 9, 9, 4, 8, 0, 0, 0, 0, 3, 7, 2, 0, 0, 0, 0, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 1, 1, 0,
            0, 5, 6, 3, 1, 6, 0, 0, 0, 0, 2, 3, 4, 7, 0, 0, 3, 9, 3, 6, 5, 8, 9, 1, 1, 3, 2, 0, 0,
            0, 0, 7, 3, 3, 0, 0, 5, 7, 0, 0, 8, 1, 9, 6, 3, 0, 0, 8, 8, 8, 8, 0,
        ];
        Solution::duplicate_zeros(&mut arr);

        assert_eq!(expected_result, arr)
    }
}

struct Solution {}
