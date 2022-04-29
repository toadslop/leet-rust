// There is a bi-directional graph with n vertices,
// where each vertex is labeled from 0 to n - 1 (inclusive).
// The edges in the graph are represented as a 2D integer
// array edges, where each edges[i] = [ui, vi] denotes a
// bi-directional edge between vertex ui and vertex vi.
// Every vertex pair is connected by at most one edge,
// and no vertex has an edge to itself.

// You want to determine if there is a valid path that
// exists from vertex source to vertex destination.

// Given edges and the integers n, source, and destination,
// return true if there is a valid path from source to
// destination, or false otherwise.

use std::collections::LinkedList;

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let end = (graph.len() as i32) - 1;
        let mut queue: LinkedList<Vec<i32>> = LinkedList::new();
        queue.push_back(vec![0]);
        let mut paths: Vec<Vec<i32>> = Vec::with_capacity(end as usize);

        while let Some(path) = queue.pop_front() {
            if let Some(&current_vertex) = path.last() {
                if let Some(neighbors) = graph.get(current_vertex as usize) {
                    for &neighbor in neighbors.iter() {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        if neighbor == end {
                            paths.push(new_path);
                        } else {
                            queue.push_back(new_path);
                        }
                    }
                }
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
        let graph = Vec::from([vec![1, 2], vec![3], vec![3], vec![]]);

        let output = Solution::all_paths_source_target(graph);
        let result: Vec<Vec<i32>> = Vec::from([[0, 1, 3], [0, 2, 3]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();

        assert_eq!(output, result);
    }

    #[test]
    fn example_2() -> () {
        let graph = Vec::from([vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]]);
        let mut output = Solution::all_paths_source_target(graph);
        output.sort();
        let mut result = Vec::from([
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ]);

        result.sort();

        assert_eq!(output, result);
    }
}
