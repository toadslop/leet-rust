use std::cmp::Ordering;

struct DisjointSet {
    group: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        DisjointSet {
            group: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, node_id: usize) -> usize {
        if self.group[node_id] != node_id {
            self.group[node_id] = self.find(self.group[node_id]);
        }
        self.group[node_id]
    }

    pub fn union(&mut self, node1_id: usize, node2_id: usize) -> bool {
        let group1_id = self.find(node1_id);
        let group2_id = self.find(node2_id);

        if group1_id == group2_id {
            return false;
        };

        match self.rank[group1_id].cmp(&self.rank[group2_id]) {
            Ordering::Less => self.group[group1_id] = group2_id,
            Ordering::Greater => self.group[group2_id] = group1_id,
            Ordering::Equal => {
                self.group[group1_id] = group2_id;
                self.rank[group2_id] += 1
            }
        }

        true
    }
}

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut all_edges = points_to_edges(&points);
        all_edges.sort_by(|a, b| a.weight.cmp(&b.weight));

        let mut ds = DisjointSet::new(points.len());
        let mut cost = 0;
        let mut used_edge_count = 0;

        for edge in all_edges.iter() {
            if ds.union(edge.pt1_id, edge.pt2_id) {
                cost += edge.weight;
                used_edge_count += 1;
            }
            if used_edge_count >= points.len() - 1 {
                break;
            }
        }

        cost
    }
}

struct WeightedEdge {
    weight: i32,
    pt1_id: usize,
    pt2_id: usize,
}

impl WeightedEdge {
    pub fn new(weight: i32, pt1_id: usize, pt2_id: usize) -> Self {
        WeightedEdge {
            weight,
            pt1_id,
            pt2_id,
        }
    }
}

fn manhattan_distance(pt1: &Vec<i32>, pt2: &Vec<i32>) -> i32 {
    (pt1[0] - pt2[0]).abs() + (pt1[1] - pt2[1]).abs()
}

fn points_to_edges(points: &Vec<Vec<i32>>) -> Vec<WeightedEdge> {
    let n = points.len();
    let mut all_edges: Vec<WeightedEdge> = Vec::with_capacity(n - 1);

    for pt1_id in (0..n).into_iter() {
        for pt2_id in (pt1_id + 1..n).into_iter() {
            let weight = manhattan_distance(&points[pt1_id], &points[pt2_id]);
            all_edges.push(WeightedEdge::new(weight, pt1_id, pt2_id));
        }
    }
    all_edges
}
