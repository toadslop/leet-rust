use std::{
    cmp::{Ordering, Reverse},
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
};

#[allow(dead_code)]
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 0;
        }
        let edges = points_to_edges(&points);
        let mut edgemap = edges_to_edgemap(&edges);
        let mut unvisited: HashSet<usize> = (0..points.len()).collect();
        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();
        let mut cost = 0;
        process_visit(&mut heap, &mut edgemap, &mut visited, &mut unvisited, 0);

        while unvisited.len() > 0 {
            let current = heap.pop().unwrap().0;
            if !visited.contains(&current.start) {
                cost += current.weight;
                process_visit(
                    &mut heap,
                    &mut edgemap,
                    &mut visited,
                    &mut unvisited,
                    current.start,
                )
            }
            if !visited.contains(&current.end) {
                cost += current.weight;
                process_visit(
                    &mut heap,
                    &mut edgemap,
                    &mut visited,
                    &mut unvisited,
                    current.end,
                )
            }
        }

        cost
    }
}

fn process_visit<'a>(
    heap: &mut BinaryHeap<Reverse<&'a Edge>>,
    edgemap: &mut HashMap<usize, Vec<&'a Edge>>,
    visited: &mut HashSet<usize>,
    unvisited: &mut HashSet<usize>,
    point: usize,
) {
    visited.insert(point);
    unvisited.remove(&point);
    for &edge in edgemap.remove(&point).unwrap().iter() {
        heap.push(Reverse(edge));
    }
}

#[derive(Debug, Eq)]
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

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
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

    #[test]
    fn example_4() -> () {
        let points = Vec::from([[0, 0]])
            .iter()
            .map(|&coord| Vec::from(coord))
            .collect();
        let output = Solution::min_cost_connect_points(points);
        assert_eq!(output, 0);
    }
}

struct Solution {}
