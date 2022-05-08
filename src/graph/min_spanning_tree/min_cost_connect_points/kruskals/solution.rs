use super::{
    edge::Edge,
    util::{connected, get_sorted_edges, union},
};

#[allow(dead_code)]
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let edges = get_sorted_edges(&points);
        let mut root = (0..points.len()).collect::<Vec<usize>>();
        let mut ranks = vec![0 as usize; points.len()];
        let mut used_count = 0;

        let mut cost = 0;
        for edge in edges.iter() {
            let Edge {
                vertex1,
                vertex2,
                weight,
            } = edge;

            if !connected(&mut root, vertex1.id, vertex2.id) {
                cost += weight;
                union(&mut root, &mut ranks, vertex1.id, vertex2.id);
                used_count += 1;
            }

            if used_count == edges.len() + 1 {
                break;
            }
        }
        cost
    }
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
