struct Solution {}

pub fn find(parents: &mut Vec<usize>, x: usize) -> usize {
    if x == parents[x] {
        return x;
    }
    parents[x] = find(parents, parents[x]);
    parents[x]
}

pub fn union(
    parents: &mut Vec<usize>,
    ranks: &mut Vec<usize>,
    x: usize,
    y: usize,
    group_count: i32,
) -> i32 {
    let root_x = find(parents, x);
    let root_y = find(parents, y);
    if root_x != root_y {
        if ranks[root_x] > ranks[root_y] {
            parents[root_y] = root_x;
        } else if ranks[root_y] > ranks[root_x] {
            parents[root_x] = root_y;
        } else {
            parents[root_y] = root_x;
            ranks[root_x] += 1;
        }
        return group_count - 1;
    }
    group_count
}

#[allow(dead_code)]
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // if n - 1 == edges.len() as i32 {
        //     return 1;
        // }
        let mut parents: Vec<usize> = (0..n as usize).collect();
        let mut ranks: Vec<usize> = vec![0; n as usize];
        let mut group_count = n;

        for edge in edges.iter() {
            group_count = union(
                &mut parents,
                &mut ranks,
                edge[0] as usize,
                edge[1] as usize,
                group_count,
            );
        }

        group_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let n = 5;
        let edges = Vec::from([[0, 1], [1, 2], [3, 4]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::count_components(n, edges);

        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() -> () {
        let n = 5;
        let edges = Vec::from([[0, 1], [1, 2], [2, 3], [3, 4]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::count_components(n, edges);

        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() -> () {
        let n = 5;
        let edges = Vec::from([[0, 1], [1, 2], [0, 2], [3, 4]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::count_components(n, edges);

        assert_eq!(result, 2);
    }
}
