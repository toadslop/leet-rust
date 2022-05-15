use std::cmp::max;

use self::{
    grid::{Grid, Point, DELTAS},
    queue::UniqueQueue,
};

#[allow(dead_code)]
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let heights = Grid(heights);
        let mut effort_table = Grid(vec![
            vec![i32::MAX; heights.get_width()];
            heights.get_height()
        ]);

        let queue: UniqueQueue<Point> = UniqueQueue::new();
        let start = Point::new(0, 0);
        effort_table.set(&start, 0);
        queue.push_back(start.clone());

        while let Some(current_point) = queue.pop_front() {
            let start_height = heights.get(&current_point).unwrap();
            for next_point in DELTAS.iter().map(|delta| *delta + current_point) {
                if let Some(current_effort) = effort_table.get(&next_point) {
                    let end_height = heights.get(&next_point).unwrap();
                    let prev_effort = effort_table.get(&current_point).unwrap();
                    let next_effort = max((start_height - end_height).abs(), prev_effort);
                    if next_effort < current_effort {
                        effort_table.set(&next_point, next_effort);
                        queue.push_back(next_point);
                    }
                }
            }
        }

        effort_table.get_last()
    }
}

pub mod grid {
    use std::ops;

    #[derive(Debug)]
    pub struct Grid(pub Vec<Vec<i32>>);

    impl Grid {
        pub fn get(&self, point: &Point) -> Option<i32> {
            if let Some(row) = self.0.get(point.y as usize) {
                if let Some(value) = row.get(point.x as usize) {
                    return Some(*value);
                };
            }
            None
        }

        pub fn set(&mut self, point: &Point, value: i32) {
            self.0[point.y as usize][point.x as usize] = value;
        }

        pub fn get_height(&self) -> usize {
            self.0.len()
        }

        pub fn get_width(&self) -> usize {
            self.0[0].len()
        }

        pub fn get_last(&self) -> i32 {
            self.0[self.0.len() - 1][self.0[0].len() - 1]
        }
    }

    pub const DELTAS: [Point; 4] = [
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
        Point::new(0, -1),
    ];

    #[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        pub const fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
    }

    impl ops::Add<Point> for Point {
        type Output = Point;

        fn add(self, rhs: Point) -> Point {
            Point::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl ops::Add<&Point> for Point {
        type Output = Point;

        fn add(self, rhs: &Point) -> Point {
            Point::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
}

pub mod queue {
    use std::cell::RefCell;
    use std::collections::{HashSet, LinkedList};
    use std::hash::Hash;

    #[derive(Debug)]
    pub struct UniqueQueue<T> {
        queue: RefCell<LinkedList<T>>,
        added: RefCell<HashSet<T>>,
    }

    impl<T> UniqueQueue<T>
    where
        T: Eq,
        T: Hash,
        T: Copy,
    {
        pub fn new() -> Self {
            UniqueQueue {
                queue: RefCell::new(LinkedList::new()),
                added: RefCell::new(HashSet::new()),
            }
        }

        pub fn push_back(&self, item: T) {
            if !self.added.borrow().contains(&item) {
                self.added.borrow_mut().insert(item.clone());
                self.queue.borrow_mut().push_back(item);
            }
        }

        pub fn pop_front(&self) -> Option<T> {
            if let Some(item) = self.queue.borrow_mut().pop_front() {
                self.added.borrow_mut().remove(&item);
                Some(item)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example_1() {
        let heights: Vec<Vec<i32>> = Vec::from([[1, 2, 2], [3, 8, 2], [5, 3, 5]])
            .iter()
            .map(|row| Vec::from(*row))
            .collect();

        let expected_result = 2;
        let actual_result = Solution::minimum_effort_path(heights);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_2() {
        let heights: Vec<Vec<i32>> = Vec::from([[1, 2, 3], [3, 8, 4], [5, 3, 5]])
            .iter()
            .map(|row| Vec::from(*row))
            .collect();

        let expected_result = 1;
        let actual_result = Solution::minimum_effort_path(heights);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_3() {
        let heights: Vec<Vec<i32>> = Vec::from([
            [1, 2, 1, 1, 1],
            [1, 2, 1, 2, 1],
            [1, 2, 1, 2, 1],
            [1, 2, 1, 2, 1],
            [1, 1, 1, 2, 1],
        ])
        .iter()
        .map(|row| Vec::from(*row))
        .collect();

        let expected_result = 0;
        let actual_result = Solution::minimum_effort_path(heights);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn example_4() {
        let heights: Vec<Vec<i32>> = Vec::from([[1, 10, 6, 7, 9, 10, 4, 9]])
            .iter()
            .map(|row| Vec::from(*row))
            .collect();

        let expected_result = 9;
        let actual_result = Solution::minimum_effort_path(heights);

        assert_eq!(expected_result, actual_result);
    }
}

struct Solution {}
