use std::cell::RefCell;

#[derive(Debug)]
struct QuickFind {
    root: RefCell<Vec<usize>>,
}

impl QuickFind {
    pub fn new(size: usize) -> Self {
        Self {
            root: RefCell::new((0..size).collect()),
        }
    }

    pub fn find(&self, mut x: usize) -> usize {
        while x != self.root.borrow()[x] {
            x = self.root.borrow()[x];
        }
        x
    }

    pub fn union(&self, x: usize, y: usize) -> () {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            self.root.borrow_mut()[root_y] = root_x
        }
    }

    pub fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let uf = QuickFind::new(10);

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
