use std::{
    cmp::{min, Reverse},
    collections::BinaryHeap,
};

#[allow(dead_code)]
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let k = k as usize - 1;
        let mut visited = vec![false; n as usize];
        let mut dist = vec![i32::MAX; n as usize];
        let edge_list: Vec<Vec<Edge>> = times.iter().fold(vec![vec![]; n as usize], into_edgelist);
        let mut pq: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        pq.push(Reverse((0, k)));
        dist[k] = i32::MIN;

        while let Some(vertex) = pq.pop() {
            let (weight, start) = vertex.0;

            if visited[start] {
                continue;
            }

            for edge in edge_list[start].iter() {
                dist[edge.end] = min(dist[edge.end], edge.weight + weight);
                pq.push(Reverse((dist[edge.end], edge.end)))
            }

            visited[start] = true;
        }

        if visited.contains(&false) {
            return -1;
        }

        *dist.iter().max().unwrap()
    }
}

fn into_edgelist(mut list: Vec<Vec<Edge>>, edge: &Vec<i32>) -> Vec<Vec<Edge>> {
    list[edge[0] as usize - 1].push(Edge::new((edge[1] - 1) as usize, edge[2]));
    list
}

#[derive(Clone, Debug)]
struct Edge {
    pub end: usize,
    pub weight: i32,
}

impl Edge {
    pub fn new(end: usize, weight: i32) -> Self {
        Edge { end, weight }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let times: Vec<Vec<i32>> = Vec::from([[2, 1, 1], [2, 3, 1], [3, 4, 1]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();
        let n = 4;
        let k = 2;

        let expected_result = 2;
        let actual_result = Solution::network_delay_time(times, n, k);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_2() {
        let times: Vec<Vec<i32>> = Vec::from([[1, 2, 1]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();
        let n = 2;
        let k = 1;

        let expected_result = 1;
        let actual_result = Solution::network_delay_time(times, n, k);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_3() {
        let times: Vec<Vec<i32>> = Vec::from([[1, 2, 1]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();
        let n = 2;
        let k = 2;

        let expected_result = -1;
        let actual_result = Solution::network_delay_time(times, n, k);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_4() {
        let times: Vec<Vec<i32>> = Vec::from([[1, 2, 1], [2, 3, 2], [1, 3, 4]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();
        let n = 3;
        let k = 1;

        let expected_result = 3;
        let actual_result = Solution::network_delay_time(times, n, k);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_5() {
        let times: Vec<Vec<i32>> = Vec::from([[1, 2, 1], [2, 3, 2], [1, 3, 1]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();
        let n = 3;
        let k = 2;

        let expected_result = -1;
        let actual_result = Solution::network_delay_time(times, n, k);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_6() {
        let times: Vec<Vec<i32>> = Vec::from([
            [2, 4, 10],
            [5, 2, 38],
            [3, 4, 33],
            [4, 2, 76],
            [3, 2, 64],
            [1, 5, 54],
            [1, 4, 98],
            [2, 3, 61],
            [2, 1, 0],
            [3, 5, 77],
            [5, 1, 34],
            [3, 1, 79],
            [5, 3, 2],
            [1, 2, 59],
            [4, 3, 46],
            [5, 4, 44],
            [2, 5, 89],
            [4, 5, 21],
            [1, 3, 86],
            [4, 1, 95],
        ])
        .iter()
        .map(|&row| Vec::from(row))
        .collect();
        let n = 5;
        let k = 1;

        let expected_result = 69;
        let actual_result = Solution::network_delay_time(times, n, k);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_7() {
        let times: Vec<Vec<i32>> = Vec::from([
            [4, 2, 76],
            [1, 3, 79],
            [3, 1, 81],
            [4, 3, 30],
            [2, 1, 47],
            [1, 5, 61],
            [1, 4, 99],
            [3, 4, 68],
            [3, 5, 46],
            [4, 1, 6],
            [5, 4, 7],
            [5, 3, 44],
            [4, 5, 19],
            [2, 3, 13],
            [3, 2, 18],
            [1, 2, 0],
            [5, 1, 25],
            [2, 5, 58],
            [2, 4, 77],
            [5, 2, 74],
        ])
        .iter()
        .map(|&row| Vec::from(row))
        .collect();
        let n = 5;
        let k = 3;

        let expected_result = 59;
        let actual_result = Solution::network_delay_time(times, n, k);
        assert_eq!(expected_result, actual_result);
    }
}

struct Solution {}
