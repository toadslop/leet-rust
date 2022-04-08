use std::collections::{HashMap, HashSet};

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(n as usize);
        let mut visited: HashSet<i32> = HashSet::with_capacity(n as usize);

        for edge in edges.iter() {
            if !graph.contains_key(&edge[0]) {
                graph.insert(edge[0], vec![]);
            }

            if !graph.contains_key(&edge[1]) {
                graph.insert(edge[1], vec![]);
            }

            if let Some(adjacent_nodes) = graph.get_mut(&edge[0]) {
                adjacent_nodes.push(edge[1]);
            };

            if let Some(adjacent_nodes) = graph.get_mut(&edge[1]) {
                adjacent_nodes.push(edge[0]);
            };
        }

        path_exists(&mut graph, &mut visited, source, destination)
    }
}

fn path_exists(
    graph: &HashMap<i32, Vec<i32>>,
    visited: &mut HashSet<i32>,
    source: i32,
    destination: i32,
) -> bool {
    if source == destination {
        return true;
    }

    if visited.contains(&source) {
        return false;
    }

    visited.insert(source);

    for vertex in graph.get(&source).unwrap().iter() {
        if path_exists(graph, visited, *vertex, destination) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let n = 3;
        let edges = [[0, 1], [1, 2], [2, 0]]
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let source = 0;
        let destination = 2;
        let result = Solution::valid_path(n, edges, source, destination);

        assert!(result);
    }

    #[test]
    fn example_2() -> () {
        let n = 6;
        let edges = [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]]
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let source = 0;
        let destination = 5;
        let result = Solution::valid_path(n, edges, source, destination);

        assert_eq!(result, false);
    }

    #[test]
    fn example_3() -> () {
        let n = 10;
        let edges = [
            [4, 3],
            [1, 4],
            [4, 8],
            [1, 7],
            [6, 4],
            [4, 2],
            [7, 4],
            [4, 0],
            [0, 9],
            [5, 4],
        ]
        .iter()
        .map(|edge| Vec::from(*edge))
        .collect();
        let source = 5;
        let destination = 9;
        let result = Solution::valid_path(n, edges, source, destination);

        assert_eq!(result, true);
    }

    #[test]
    fn example_4() -> () {
        let n = 10;
        let edges = [
            [0, 7],
            [0, 8],
            [6, 1],
            [2, 0],
            [0, 4],
            [5, 8],
            [4, 7],
            [1, 3],
            [3, 5],
            [6, 5],
        ]
        .iter()
        .map(|edge| Vec::from(*edge))
        .collect();
        let source = 7;
        let destination = 5;
        let result = Solution::valid_path(n, edges, source, destination);

        assert_eq!(result, true);
    }
}
