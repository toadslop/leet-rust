struct Solution {}
#[allow(dead_code)]
impl Solution {
    pub fn earliest_acq(mut logs: Vec<Vec<i32>>, n: i32) -> i32 {
        logs.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut roots: Vec<usize> = (0..n as usize).collect();
        let mut ranks: Vec<usize> = vec![0; n as usize];
        let mut groups = n;

        for log in logs.iter() {
            groups = union(&mut roots, &mut ranks, log, groups);
            if groups == 1 {
                return log[0];
            }
        }

        -1
    }
}

fn find(roots: &mut Vec<usize>, x: usize) -> usize {
    if x == roots[x] {
        return x;
    }
    roots[x] = find(roots, roots[x]);
    roots[x]
}

fn union(roots: &mut Vec<usize>, ranks: &mut Vec<usize>, edge: &Vec<i32>, groups: i32) -> i32 {
    let root_x = find(roots, edge[1] as usize);
    let root_y = find(roots, edge[2] as usize);

    if root_x != root_y {
        if ranks[root_x] > ranks[root_y] {
            roots[root_y] = root_x;
        } else {
            roots[root_x] = root_y;
            if ranks[root_x] == ranks[root_y] {
                ranks[root_y] += 1;
            }
        }
        return groups - 1;
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let n = 6;
        let logs = Vec::from([
            [20190101, 0, 1],
            [20190104, 3, 4],
            [20190107, 2, 3],
            [20190211, 1, 5],
            [20190224, 2, 4],
            [20190301, 0, 3],
            [20190312, 1, 2],
            [20190322, 4, 5],
        ])
        .iter()
        .map(|edge| Vec::from(*edge))
        .collect();
        let result = Solution::earliest_acq(logs, n);

        assert_eq!(result, 20190301);
    }

    #[test]
    fn example_2() -> () {
        let n = 4;
        let logs = Vec::from([[0, 2, 0], [1, 0, 1], [3, 0, 3], [4, 1, 2], [7, 3, 1]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::earliest_acq(logs, n);

        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() -> () {
        let n = 4;
        let logs = Vec::from([
            [9, 3, 0],
            [0, 2, 1],
            [8, 0, 1],
            [1, 3, 2],
            [2, 2, 0],
            [3, 3, 1],
        ])
        .iter()
        .map(|edge| Vec::from(*edge))
        .collect();
        let result = Solution::earliest_acq(logs, n);

        assert_eq!(result, 2);
    }
}
