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

use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let adj_map = edges
            .iter()
            .fold(HashMap::with_capacity(n as usize), into_adj_list);
        let mut visited = HashSet::with_capacity(n as usize);
        let mut queue = VecDeque::with_capacity(n as usize);
        queue.push_back(&source);

        while let Some(vertex) = queue.pop_front() {
            if vertex == &destination {
                return true;
            }
            if visited.contains(&vertex) {
                continue;
            }
            visited.insert(vertex);

            if let Some(neighbors) = adj_map.get(&vertex) {
                for neighbor in neighbors.iter() {
                    queue.push_back(neighbor);
                }
            }
        }

        false
    }
}

fn into_adj_list<'a>(
    mut map: HashMap<&'a i32, Vec<&'a i32>>,
    edge: &'a Vec<i32>,
) -> HashMap<&'a i32, Vec<&'a i32>> {
    process_edge(&mut map, edge, false);
    process_edge(&mut map, edge, true);
    map
}

fn process_edge<'a>(map: &mut HashMap<&'a i32, Vec<&'a i32>>, edge: &'a Vec<i32>, reverse: bool) {
    let (start_idx, end_idx) = if reverse { (1, 0) } else { (0, 1) };
    match map.entry(&edge[start_idx]) {
        Entry::Occupied(mut entry) => {
            entry.get_mut().push(&edge[end_idx]);
        }
        Entry::Vacant(entry) => {
            entry.insert(vec![&edge[end_idx]]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let n = 3;
        let edges = Vec::from([[0, 1], [1, 2], [2, 0]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let source = 0;
        let destination = 2;

        let output = Solution::valid_path(n, edges, source, destination);

        assert_eq!(output, true);
    }

    #[test]
    fn example_2() -> () {
        let n = 6;
        let edges = Vec::from([[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let source = 0;
        let destination = 5;

        let output = Solution::valid_path(n, edges, source, destination);

        assert_eq!(output, false);
    }

    #[test]
    fn example_3() -> () {
        let n = 10;
        let edges = Vec::from([
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
        ])
        .iter()
        .map(|edge| Vec::from(*edge))
        .collect();
        let source = 7;
        let destination = 5;

        let output = Solution::valid_path(n, edges, source, destination);

        assert_eq!(output, true);
    }
}
