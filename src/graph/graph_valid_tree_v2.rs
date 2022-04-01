use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct DisjointSet {
    is_tree: RefCell<bool>,
    rank: RefCell<Vec<usize>>,
    root: RefCell<Vec<usize>>,
    parents: RefCell<Vec<Option<usize>>>,
    group_count: Cell<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            is_tree: RefCell::new(true),
            rank: RefCell::new(vec![1; size]),
            root: RefCell::new((0..size).collect()),
            parents: RefCell::new(vec![None; size]),
            group_count: Cell::new(size),
        }
    }

    pub fn find(&self, mut x: usize) -> usize {
        let mut root = self.root.borrow_mut();
        if x == root[x] {
            return x;
        }

        let mut stack: Vec<usize> = Vec::with_capacity(root.len());

        while x != root[x] {
            stack.push(x);
            x = root[x];
        }

        while let Some(i) = stack.pop() {
            root[i] = x;
        }

        x
    }

    pub fn union(&self, x: usize, y: usize) -> () {
        let mut parents = self.parents.borrow_mut();
        match parents[y] {
            Some(_) => match parents[x] {
                Some(_) => {
                    self.is_tree.replace(false);
                }
                None => parents[x] = Some(y),
            },
            None => parents[y] = Some(x),
        };

        if !parents.contains(&None) {
            self.is_tree.replace(false);
        }

        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            let mut rank = self.rank.borrow_mut();
            let mut root = self.root.borrow_mut();
            if rank[root_x] > rank[root_y] {
                root[root_y] = root_x;
            } else if rank[root_y] > rank[root_x] {
                root[root_x] = root_y;
            } else {
                root[root_y] = root_x;
                rank[root_x] += 1;
            }
            self.group_count.replace(self.group_count.get() - 1);
        }
    }

    pub fn get_count(&self) -> i32 {
        self.group_count
            .get()
            .try_into()
            .expect("usize cannot be convered to i32")
    }
}

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let ds = DisjointSet::new(n.try_into().unwrap());
        println!("{:?}", ds);
        for edge in edges.iter() {
            ds.union(edge[0].try_into().unwrap(), edge[1].try_into().unwrap());
            println!("{:?}", ds);
            if ds.is_tree.borrow().eq(&false) {
                return false;
            }
        }
        ds.get_count() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let n = 5;
        let edges = Vec::from([[0, 1], [0, 2], [0, 3], [1, 4]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::valid_tree(n, edges);

        assert!(result);
    }

    #[test]
    fn example_2() -> () {
        let n = 5;
        let edges = Vec::from([[0, 1], [1, 2], [2, 3], [1, 3], [1, 4]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::valid_tree(n, edges);

        assert_eq!(result, false);
    }

    #[test]
    fn example_3() -> () {
        let n = 4;
        let edges = Vec::from([[0, 1], [2, 3]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::valid_tree(n, edges);

        assert_eq!(result, false);
    }

    #[test]
    fn example_4() -> () {
        let n = 3;
        let edges = Vec::from([[1, 0], [2, 0]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::valid_tree(n, edges);

        assert_eq!(result, true);
    }

    #[test]
    fn example_5() -> () {
        let n = 3;
        let edges = Vec::from([[2, 0], [2, 1]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::valid_tree(n, edges);

        assert_eq!(result, true);
    }

    #[test]
    fn example_6() -> () {
        let n = 3;
        let edges = Vec::from([[1, 0], [0, 2], [2, 1]])
            .iter()
            .map(|edge| Vec::from(*edge))
            .collect();
        let result = Solution::valid_tree(n, edges);

        assert_eq!(result, false);
    }
}
