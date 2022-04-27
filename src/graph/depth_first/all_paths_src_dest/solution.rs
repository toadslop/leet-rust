use super::{graph::Graph, traverse::traverse};

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn leads_to_destination(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
    ) -> bool {
        let mut graph = Graph::with_capacity(n);
        graph.add_edges(edges);

        traverse(
            &graph,
            &mut vec![Color::White; n as usize],
            source as usize,
            destination as usize,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    White,
    Grey,
    Black,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let n = 3;
        let edges: Vec<Vec<i32>> = Vec::from([[0, 1], [0, 2]])
            .iter()
            .map(|&edge| Vec::from(edge))
            .collect();
        let source = 0;
        let destination = 2;

        let output = Solution::leads_to_destination(n, edges, source, destination);

        assert_eq!(output, false);
    }

    #[test]
    fn example_2() -> () {
        let n = 4;
        let edges: Vec<Vec<i32>> = Vec::from([[0, 1], [0, 3], [1, 2], [2, 1]])
            .iter()
            .map(|&edge| Vec::from(edge))
            .collect();
        let source = 0;
        let destination = 3;

        let output = Solution::leads_to_destination(n, edges, source, destination);

        assert_eq!(output, false);
    }

    #[test]
    fn example_3() -> () {
        let n = 4;
        let edges: Vec<Vec<i32>> = Vec::from([[0, 1], [0, 2], [1, 3], [2, 3]])
            .iter()
            .map(|&edge| Vec::from(edge))
            .collect();
        let source = 0;
        let destination = 3;

        let output = Solution::leads_to_destination(n, edges, source, destination);

        assert_eq!(output, true);
    }

    #[test]
    fn example_4() -> () {
        let n = 2;
        let edges: Vec<Vec<i32>> = Vec::from([[0, 1], [1, 1]])
            .iter()
            .map(|&edge| Vec::from(edge))
            .collect();
        let source = 0;
        let destination = 1;

        let output = Solution::leads_to_destination(n, edges, source, destination);

        assert_eq!(output, false);
    }

    #[test]
    fn example_5() -> () {
        let n = 5;
        let edges: Vec<Vec<i32>> = Vec::from([
            [0, 1],
            [0, 2],
            [0, 3],
            [0, 3],
            [1, 2],
            [1, 3],
            [1, 4],
            [2, 3],
            [2, 4],
            [3, 4],
        ])
        .iter()
        .map(|&edge| Vec::from(edge))
        .collect();
        let source = 0;
        let destination = 4;

        let output = Solution::leads_to_destination(n, edges, source, destination);

        assert_eq!(output, true);
    }

    #[test]
    fn example_6() -> () {
        let n = 3;
        let edges: Vec<Vec<i32>> = Vec::from([[0, 1], [1, 1], [1, 2]])
            .iter()
            .map(|&edge| Vec::from(edge))
            .collect();
        let source = 0;
        let destination = 2;

        let output = Solution::leads_to_destination(n, edges, source, destination);

        assert_eq!(output, false);
    }
}
