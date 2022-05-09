use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

#[allow(dead_code)]
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 0 {
            return 0;
        }

        let size = points.len();
        let mut pq = BinaryHeap::with_capacity(size);
        let mut visited = vec![false; size];
        let mut result = 0;
        let mut count = size - 1;

        process_pq(&points, 0, &mut visited, size, &mut pq);

        while !pq.is_empty() && count > 0 {
            let edge = pq.pop().unwrap().0;
            let end = edge.end;
            let cost = edge.cost;

            if !visited[end] {
                result += cost;
                process_pq(&points, end, &mut visited, size, &mut pq);
                count -= 1;
            }
        }

        result
    }
}

fn process_pq(
    points: &Vec<Vec<i32>>,
    i: usize,
    visited: &mut Vec<bool>,
    size: usize,
    pq: &mut BinaryHeap<Reverse<Edge>>,
) {
    let coordinate1 = &points[i];
    for j in (1..size).into_iter() {
        pq.push(Reverse(Edge::new(
            j,
            manhattan_distance(&coordinate1, &points[j]),
        )));
    }
    visited[i] = true;
}

// struct PrimsTracker<T> {
//     pq: BinaryHeap<T>,
//     visited: Vec<bool>,
//     count: usize,
// }

// impl<T> PrimsTracker<T>
// where
//     T: Ord,
// {
//     pub fn with_capacity(capacity: usize) -> Self {
//         PrimsTracker {
//             pq: BinaryHeap::with_capacity(capacity),
//             visited: vec![false; capacity],
//             count: capacity - 1,
//         }
//     }

//     pub fn is_finished(&self) -> bool {
//         self.pq.is_empty() && self.count > 0
//     }
// }

#[derive(Debug, Eq)]
struct Edge {
    end: usize,
    cost: i32,
}

impl Edge {
    pub fn new(end: usize, cost: i32) -> Self {
        Edge { end, cost }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
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
