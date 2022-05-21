#[allow(dead_code)]
impl Solution {
    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 3;
        let relations: Vec<Vec<i32>> = Vec::from([[1, 3], [2, 3]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let expected_result = 2;
        let actual_result = Solution::minimum_semesters(n, relations);

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn example_2() {
        let n = 3;
        let relations: Vec<Vec<i32>> = Vec::from([[1, 2], [2, 3], [3, 1]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let expected_result = -1;
        let actual_result = Solution::minimum_semesters(n, relations);

        assert_eq!(expected_result, actual_result)
    }
}

struct Solution {}
