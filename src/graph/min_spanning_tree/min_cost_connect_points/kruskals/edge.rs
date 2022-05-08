use super::{point::Point, util::manhattan_distance};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    pub vertex1: Point,
    pub vertex2: Point,
    pub weight: i32,
}

impl Edge {
    pub fn new(pt1: &Vec<i32>, pt2: &Vec<i32>, id1: usize, id2: usize) -> Self {
        Self {
            vertex1: Point {
                x: pt1[0],
                y: pt1[1],
                id: id1,
            },
            vertex2: Point {
                x: pt2[0],
                y: pt2[1],
                id: id2,
            },
            weight: manhattan_distance(pt1, pt2),
        }
    }
}
