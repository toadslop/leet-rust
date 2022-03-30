use crate::graph::disjoint_set::disjoint_set::DisjointSet;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    const CONNECTED: i32 = 1;

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let ds = DisjointSet::new(is_connected.len());

        for (i, row) in is_connected.iter().enumerate() {
            for (j, connected) in row.iter().enumerate() {
                if connected.eq(&Self::CONNECTED) {
                    ds.union(i, j);
                }
            }
        }

        ds.get_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let result = Solution::find_circle_num(is_connected);

        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() -> () {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let result = Solution::find_circle_num(is_connected);

        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() -> () {
        let is_connected = vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
        ];
        let result = Solution::find_circle_num(is_connected);

        assert_eq!(result, 1);
    }
}
