use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let n = n as usize;
        let mut neighbors: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        let mut leaves: Vec<i32> = Vec::new();

        for edge in edges.iter() {
            let i = edge[0] as usize;
            let j = edge[1] as usize;
            neighbors.get_mut(i).and_then(|set| Some(set.insert(j)));
            neighbors.get_mut(j).and_then(|set| Some(set.insert(i)));
        }

        for (node, list) in neighbors.iter().enumerate() {
            if list.len() == 1 {
                leaves.push(node as i32);
            }
        }

        let mut remaining_nodes = n;
        while remaining_nodes > 2 {
            remaining_nodes -= leaves.len();
            let mut new_leaves: Vec<i32> = Vec::new();

            for &leaf in leaves.iter() {
                let neighbor = neighbors
                    .get(leaf as usize)
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap()
                    .to_owned();

                neighbors
                    .get_mut(neighbor)
                    .unwrap()
                    .remove(&(leaf as usize));

                if neighbors.get(neighbor).unwrap().len() == 1 {
                    new_leaves.push(neighbor as i32);
                }
            }
            leaves = new_leaves;
        }

        leaves
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let n = 4;
        let edges: Vec<Vec<i32>> = Vec::from([[1, 0], [1, 2], [1, 3]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let mut expected_result = Vec::from([1]);
        expected_result.sort();

        let mut actual_result = Solution::find_min_height_trees(n, edges);
        actual_result.sort();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_2() {
        let n = 6;
        let edges: Vec<Vec<i32>> = Vec::from([[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]])
            .iter()
            .map(|arr| Vec::from(*arr))
            .collect();

        let mut expected_result = Vec::from([3, 4]);
        expected_result.sort();

        let mut actual_result = Solution::find_min_height_trees(n, edges);
        actual_result.sort();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_3() {
        let n = 1;
        let edges: Vec<Vec<i32>> = Vec::from([]);

        let mut expected_result = Vec::from([0]);
        expected_result.sort();

        let mut actual_result = Solution::find_min_height_trees(n, edges);
        actual_result.sort();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example4() {
        let n = 2;
        let edges: Vec<Vec<i32>> = Vec::from([vec![0, 1]]);

        let mut expected_result = Vec::from([0, 1]);
        expected_result.sort();

        let mut actual_result = Solution::find_min_height_trees(n, edges);
        actual_result.sort();

        assert_eq!(expected_result, actual_result);
    }
}

#[allow(dead_code)]
struct Solution {}
