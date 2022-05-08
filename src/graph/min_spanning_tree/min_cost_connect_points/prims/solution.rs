use std::collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet};

#[allow(dead_code)]
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let edges = points_to_edges(&points);
        println!("EDGES {:?}", edges);
        println!("");
        let edgemap = edges_to_edgemap(&edges);
        println!("EDGEMAP {:?}", edgemap);
        let unvisited: HashSet<usize> = (0..points.len()).collect();
        let visited: HashSet<usize> = HashSet::new();
        let mut heap: BinaryHeap<(i32, [usize; 2])> = BinaryHeap::new();
        heap.push((2, [2, 2]));
        unimplemented!()
    }
}

#[derive(Debug)]
struct Edge {
    start: usize,
    end: usize,
    weight: i32,
}

impl Edge {
    pub fn new(start: usize, end: usize, weight: i32) -> Self {
        Edge { start, end, weight }
    }
}

fn points_to_edges(points: &Vec<Vec<i32>>) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::with_capacity(points.len() - 1);
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            edges.push(Edge::new(i, j, manhattan_distance(&points[i], &points[j])));
        }
    }
    edges
}

fn edges_to_edgemap<'a>(edges: &'a Vec<Edge>) -> HashMap<usize, Vec<&'a Edge>> {
    let mut edgemap: HashMap<usize, Vec<&Edge>> = HashMap::with_capacity(edges.len());
    for edge in edges.iter() {
        process_edgemap_entry(&mut edgemap, edge, End::Start);
        process_edgemap_entry(&mut edgemap, edge, End::End);
    }
    edgemap
}

fn process_edgemap_entry<'a>(
    edgemap: &mut HashMap<usize, Vec<&'a Edge>>,
    edge: &'a Edge,
    end: End,
) {
    let key = match end {
        End::End => edge.end,
        End::Start => edge.start,
    };
    match edgemap.entry(key) {
        Entry::Occupied(mut entry) => {
            entry.get_mut().push(&edge);
        }
        Entry::Vacant(entry) => {
            entry.insert(vec![edge]);
        }
    };
}

enum End {
    Start,
    End,
}

fn manhattan_distance(start: &Vec<i32>, end: &Vec<i32>) -> i32 {
    (start[0] - end[0]).abs() + (start[1] - end[1]).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let points = Vec::from([[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]])
            .iter()
            .map(|&coord| Vec::from(coord))
            .collect();
        let output = Solution::min_cost_connect_points(points);
        assert_eq!(output, 20);
    }

    #[test]
    fn example_2() -> () {
        let points = Vec::from([[3, 12], [-2, 5], [-4, 1]])
            .iter()
            .map(|&coord| Vec::from(coord))
            .collect();
        let output = Solution::min_cost_connect_points(points);
        assert_eq!(output, 18);
    }

    #[test]
    fn example_3() -> () {
        let points = Vec::from([[2, -3], [-17, -8], [13, 8], [-17, -15]])
            .iter()
            .map(|&coord| Vec::from(coord))
            .collect();
        let output = Solution::min_cost_connect_points(points);
        assert_eq!(output, 53);
    }
}

struct Solution {}
