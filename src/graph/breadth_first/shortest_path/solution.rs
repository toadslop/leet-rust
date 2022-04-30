use std::collections::{HashSet, LinkedList};

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = LinkedList::new();
        let target = (grid.len() as i32 - 1, grid[0].len() as i32 - 1);
        let directions = [
            Direction::SouthEast,
            Direction::East,
            Direction::South,
            Direction::North,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::West,
            Direction::SouthWest,
        ];
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        queue.push_back((-1, -1, 0));

        while let Some((x, y, dist)) = queue.pop_front() {
            if x == target.0 && y == target.1 {
                return dist;
            }
            for direction in directions.iter() {
                if let Some((x, y)) = next_position(direction, (x, y), &grid) {
                    if !visited.contains(&(x, y)) {
                        queue.push_back((x, y, dist + 1));
                        visited.insert((x, y));
                    }
                }
            }
        }
        NO_PATH
    }
}

const NO_PATH: i32 = -1;

fn next_position(dir: &Direction, (x, y): (i32, i32), grid: &Vec<Vec<i32>>) -> Option<(i32, i32)> {
    match dir {
        Direction::North => get_position(x, y - 1, grid),
        Direction::NorthEast => get_position(x + 1, y - 1, grid),
        Direction::East => get_position(x + 1, y, grid),
        Direction::SouthEast => get_position(x + 1, y + 1, grid),
        Direction::South => get_position(x, y + 1, grid),
        Direction::SouthWest => get_position(x - 1, y + 1, grid),
        Direction::West => get_position(x - 1, y, grid),
        Direction::NorthWest => get_position(x - 1, y - 1, grid),
    }
}

fn get_position(x: i32, y: i32, grid: &Vec<Vec<i32>>) -> Option<(i32, i32)> {
    if let Some(row) = grid.get(y as usize) {
        if let Some(point) = row.get(x as usize) {
            if point == &0 {
                return Some((x, y));
            }
        }
    }
    None
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> () {
        let grid = Vec::from([[0, 1], [1, 0]])
            .iter()
            .map(|&row| row.into())
            .collect();

        let output = Solution::shortest_path_binary_matrix(grid);

        assert_eq!(output, 2);
    }

    #[test]
    fn example_2() -> () {
        let grid = Vec::from([[0, 0, 0], [1, 1, 0], [1, 1, 0]])
            .iter()
            .map(|&row| row.into())
            .collect();

        let output = Solution::shortest_path_binary_matrix(grid);

        assert_eq!(output, 4);
    }

    #[test]
    fn example_3() -> () {
        let grid = Vec::from([[1, 0, 0], [1, 1, 0], [1, 1, 0]])
            .iter()
            .map(|&row| row.into())
            .collect();

        let output = Solution::shortest_path_binary_matrix(grid);

        assert_eq!(output, -1);
    }

    #[test]
    fn example_4() -> () {
        let grid = Vec::from([
            [0, 1, 1, 0, 0, 0],
            [0, 1, 0, 1, 1, 0],
            [0, 1, 1, 0, 1, 0],
            [0, 0, 0, 1, 1, 0],
            [1, 1, 1, 1, 1, 0],
            [1, 1, 1, 1, 1, 0],
        ])
        .iter()
        .map(|&row| row.into())
        .collect();

        let output = Solution::shortest_path_binary_matrix(grid);

        assert_eq!(output, 14);
    }
}
