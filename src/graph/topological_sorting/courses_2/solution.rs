use std::collections::LinkedList;

#[allow(dead_code)]
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        if num_courses == 1 {
            return vec![0];
        }

        let num_courses = num_courses as usize;
        let mut in_degrees = vec![0; num_courses];
        let mut edge_list: Vec<Vec<usize>> = vec![vec![]; num_courses];
        let mut queue: LinkedList<usize> = LinkedList::new();
        let mut result: Vec<i32> = Vec::with_capacity(num_courses);

        for prereq_arr in prerequisites.iter() {
            let course_id = prereq_arr[0] as usize;
            let prereq_id = prereq_arr[1] as usize;
            in_degrees[course_id] += 1;
            edge_list[prereq_id].push(course_id);
        }

        for (course_id, count) in in_degrees.iter().enumerate() {
            if *count == 0 {
                queue.push_back(course_id);
            }
        }

        while let Some(course_id) = queue.pop_front() {
            result.push(course_id as i32);

            for &next_id in edge_list[course_id].iter() {
                in_degrees[next_id] -= 1;
                if in_degrees[next_id] == 0 {
                    queue.push_back(next_id);
                }
            }
        }

        if result.len() == num_courses {
            result
        } else {
            vec![]
        }
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

        let expected_result_options: Vec<Vec<i32>> =
            Vec::from([[0, 2, 1, 3], [0, 2, 1, 3], [0, 1, 2, 3]])
                .iter()
                .map(|arr| Vec::from(*arr))
                .collect();
        let actual_result = Solution::find_order(num_courses, prerequisites);
        println!("{:?}", actual_result);
        assert!(expected_result_options.contains(&actual_result))
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

    #[test]
    fn example_4() {
        let num_courses = 3;
        let prerequisites: Vec<Vec<i32>> = Vec::from([[1, 0], [1, 2], [0, 1]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let expected_result = Vec::from([]);
        let actual_result = Solution::find_order(num_courses, prerequisites);

        assert_eq!(actual_result, expected_result)
    }
}

struct Solution {}
