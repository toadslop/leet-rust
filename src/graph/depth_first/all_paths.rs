struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut stack: Vec<Vec<i32>> = vec![vec![0]];
        let mut paths = vec![];

        loop {
            let mut new_paths: Vec<Vec<i32>> = vec![];

            for path in stack.iter() {
                if let Some(&last) = &path.last() {
                    if let Some(edges) = &graph.get(last as usize) {
                        for &vertex in edges.iter() {
                            let mut new_path = path.clone();
                            new_path.push(vertex);
                            if vertex == graph.len() as i32 - 1 {
                                paths.push(new_path);
                            } else {
                                new_paths.push(new_path);
                            }
                        }
                    }
                }
            }

            match new_paths.len() {
                0 => break,
                _ => stack = new_paths,
            }
        }
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];

        let mut output = [[0, 1, 3], [0, 2, 3]]
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect::<Vec<Vec<i32>>>();
        output.sort();

        let mut result = Solution::all_paths_source_target(graph);
        result.sort();

        assert_eq!(result, output);
    }

    #[test]
    fn example_2() -> () {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];

        let mut output = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];
        output.sort();
        let mut result = Solution::all_paths_source_target(graph);
        result.sort();
        assert_eq!(result, output);
    }
}
