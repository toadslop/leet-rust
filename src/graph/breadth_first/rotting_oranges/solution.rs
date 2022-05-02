use std::collections::LinkedList;

#[allow(dead_code)]
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut rotten = LinkedList::new();
        let mut fresh_count = 0;
        let mut time = 0;

        grid.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .for_each(|(x, &space)| match space.into() {
                    Space::Rotten => rotten.push_back((x, y, 0)),
                    Space::Fresh => fresh_count += 1,
                    Space::Empty => (),
                });
        });

        while let Some((x, y, depth)) = rotten.pop_front() {
            for [delta_x, delta_y] in DELTAS.iter() {
                let i = delta_x + x as isize;
                let j = delta_y + y as isize;
                if in_bounds(&grid, i, j) {
                    if Space::from(grid[j as usize][i as usize]) == Space::Fresh {
                        grid[j as usize][i as usize] = Space::Rotten.into();
                        rotten.push_back((i as usize, j as usize, depth + 1));
                        fresh_count -= 1;
                    }
                }
            }

            if depth > time {
                time = depth;
            }
        }

        if fresh_count != 0 {
            -1
        } else {
            time
        }
    }
}

fn in_bounds(grid: &Vec<Vec<i32>>, x: isize, y: isize) -> bool {
    if y >= 0 && y < grid.len() as isize {
        if x >= 0 && x < grid[y as usize].len() as isize {
            return true;
        }
    };
    false
}

const DELTAS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

#[derive(PartialEq, Debug)]
enum Space {
    Fresh,
    Rotten,
    Empty,
}

impl From<i32> for Space {
    fn from(index: i32) -> Space {
        match index {
            0 => Space::Empty,
            1 => Space::Fresh,
            2 => Space::Rotten,
            _ => panic!("Can only convert 0, 1, or 2 to Space"),
        }
    }
}

impl From<Space> for i32 {
    fn from(space: Space) -> i32 {
        match space {
            Space::Empty => 0,
            Space::Fresh => 1,
            Space::Rotten => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let grid = Vec::from([[2, 1, 1], [1, 1, 0], [0, 1, 1]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();

        let output = Solution::oranges_rotting(grid);

        assert_eq!(output, 4);
    }

    #[test]
    fn example_2() -> () {
        let grid = Vec::from([[2, 1, 1], [0, 1, 1], [1, 0, 1]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();

        let output = Solution::oranges_rotting(grid);

        assert_eq!(output, -1);
    }

    #[test]
    fn example_3() -> () {
        let grid = Vec::from([[0, 2]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();

        let output = Solution::oranges_rotting(grid);

        assert_eq!(output, 0);
    }

    #[test]
    fn example_4() -> () {
        let grid = Vec::from([[1]]).iter().map(|&row| Vec::from(row)).collect();

        let output = Solution::oranges_rotting(grid);

        assert_eq!(output, -1);
    }

    #[test]
    fn example_5() -> () {
        let grid = Vec::from([[1, 2]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();

        let output = Solution::oranges_rotting(grid);

        assert_eq!(output, 1);
    }

    #[test]
    fn example_6() -> () {
        let grid = Vec::from([[0]]).iter().map(|&row| Vec::from(row)).collect();

        let output = Solution::oranges_rotting(grid);

        assert_eq!(output, 0);
    }

    #[test]
    fn example_7() -> () {
        let grid = Vec::from([[2, 2], [1, 1], [0, 0], [2, 0]])
            .iter()
            .map(|&row| Vec::from(row))
            .collect();

        let output = Solution::oranges_rotting(grid);

        assert_eq!(output, 1);
    }
}

struct Solution {}
