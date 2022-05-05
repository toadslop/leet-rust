#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub id: usize,
}

impl Point {
    pub fn new(pt: Vec<i32>, id: usize) -> Self {
        Point {
            x: pt[0],
            y: pt[1],
            id,
        }
    }
}
