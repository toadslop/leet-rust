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

struct Solution {}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        unimplemented!()
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
}
