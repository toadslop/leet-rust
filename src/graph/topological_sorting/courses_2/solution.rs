#[allow(dead_code)]
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let num_courses = 2;
        let prerequisites: Vec<Vec<i32>> = Vec::from([[1, 0]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let expected_result = Vec::from([0, 1]);
        let actual_result = Solution::find_order(num_courses, prerequisites);

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_2() {
        let num_courses = 4;
        let prerequisites: Vec<Vec<i32>> = Vec::from([[1, 0], [2, 0], [3, 1], [3, 2]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let expected_result = Vec::from([0, 2, 1, 3]);
        let actual_result = Solution::find_order(num_courses, prerequisites);

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_3() {
        let num_courses = 1;
        let prerequisites: Vec<Vec<i32>> =
            Vec::from([[]]).iter().map(|arr| Vec::from(*arr)).collect();

        let expected_result = Vec::from([0]);
        let actual_result = Solution::find_order(num_courses, prerequisites);

        assert_eq!(actual_result, expected_result)
    }
}

struct Solution {}
