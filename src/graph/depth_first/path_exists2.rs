struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut root = (0..n as usize).collect::<Vec<usize>>();
        let mut rank = vec![0; n as usize];

        for edge in edges.iter() {
            union(&mut root, &mut rank, edge);
        }

        connected(&mut root, source as usize, destination as usize)
    }
}

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if x != root[x] {
        root[x] = find(root, root[x]);
    }
    root[x]
}

fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, edge: &Vec<i32>) -> () {
    let root_x = find(root, edge[0] as usize);
    let root_y = find(root, edge[1] as usize);

    if root_x != root_y {
        if rank[root_x] > rank[root_y] {
            root[root_y] = root_x;
        } else {
            root[root_x] = root_y;
            if rank[root_x] == rank[root_y] {
                rank[root_y] += 1;
            }
        }
    }
}

fn connected(root: &mut Vec<usize>, x: usize, y: usize) -> bool {
    find(root, x) == find(root, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let n = 3;
        let edges = [[0, 1], [1, 2], [2, 0]]
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let source = 0;
        let destination = 2;
        let result = Solution::valid_path(n, edges, source, destination);

        assert!(result);
    }

    #[test]
    fn example_2() -> () {
        let n = 6;
        let edges = [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]]
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let source = 0;
        let destination = 5;
        let result = Solution::valid_path(n, edges, source, destination);

        assert_eq!(result, false);
    }

    #[test]
    fn example_3() -> () {
        let n = 10;
        let edges = [
            [4, 3],
            [1, 4],
            [4, 8],
            [1, 7],
            [6, 4],
            [4, 2],
            [7, 4],
            [4, 0],
            [0, 9],
            [5, 4],
        ]
        .iter()
        .map(|edge| Vec::from(*edge))
        .collect();
        let source = 5;
        let destination = 9;
        let result = Solution::valid_path(n, edges, source, destination);

        assert_eq!(result, true);
    }

    #[test]
    fn example_4() -> () {
        let n = 10;
        let edges = [
            [0, 7],
            [0, 8],
            [6, 1],
            [2, 0],
            [0, 4],
            [5, 8],
            [4, 7],
            [1, 3],
            [3, 5],
            [6, 5],
        ]
        .iter()
        .map(|edge| Vec::from(*edge))
        .collect();
        let source = 7;
        let destination = 5;
        let result = Solution::valid_path(n, edges, source, destination);

        assert_eq!(result, true);
    }
}
