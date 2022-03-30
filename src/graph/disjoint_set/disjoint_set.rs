use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct DisjointSet {
    root: RefCell<Vec<usize>>,
    rank: RefCell<Vec<usize>>,
    group_count: Cell<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            root: RefCell::new((0..size).collect()),
            rank: RefCell::new(vec![1; size]),
            group_count: Cell::new(size),
        }
    }

    pub fn find2(&self, x: usize) -> usize {
        let mut root = self.root.borrow_mut();
        self._find(x, &mut root)
    }

    fn _find(&self, x: usize, root: &mut Vec<usize>) -> usize {
        if x == root[x] {
            return x;
        }
        root[x] = self._find(root[x], root);
        root[x]
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
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            self.group_count.replace(self.group_count.get() - 1);
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
        }
    }

    pub fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn get_count(&self) -> i32 {
        self.group_count
            .get()
            .try_into()
            .expect("usize cannot be convered to i32")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let uf = DisjointSet::new(10);

        uf.union(1, 2);
        uf.union(2, 5);
        uf.union(5, 6);
        uf.union(6, 7);
        uf.union(3, 8);
        uf.union(8, 9);

        assert!(uf.connected(1, 5));
        assert!(uf.connected(1, 5));
        assert!(uf.connected(5, 7));
        assert_eq!(uf.connected(4, 9), false);

        uf.union(9, 4);
        assert!(uf.connected(4, 9));
    }
}
