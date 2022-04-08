struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut stack: Vec<Vec<i32>> = vec![vec![0]];
        let mut paths: Vec<Vec<i32>> = vec![];

        loop {
            match stack.pop().as_mut() {
                Some(path) => match path.last() {
                    Some(last) => {
                        if *last == (graph.len() - 1) as i32 {
                            paths.push(path.to_owned());
                        } else {
                            for child in graph[*last as usize].iter() {
                                let mut next_path = path.clone();
                                next_path.push(*child);
                                stack.push(next_path.to_vec())
                            }
                        }
                    }
                    None => break,
                },
                None => break,
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
