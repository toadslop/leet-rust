use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug)]
pub struct QuickFind {
    root: RefCell<Vec<usize>>,
}

#[allow(dead_code)]
impl QuickFind {
    pub fn new(size: usize) -> Self {
        Self {
            root: RefCell::new((0..size).collect()),
        }
    }

    pub fn find(&self, x: usize) -> usize {
        self.root.borrow()[x]
    }

    pub fn union(&self, x: usize, y: usize) -> () {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            self.root.replace_with(|root| {
                root.iter()
                    .map(|i| if *i == root_y { root_x } else { *i })
                    .collect()
            });
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
