use super::edge::Edge;

pub fn manhattan_distance(pt1: &Vec<i32>, pt2: &Vec<i32>) -> i32 {
    (pt1[0] - pt2[0]).abs() + (pt1[1] - pt2[1]).abs()
}

pub fn get_sorted_edges(points: &Vec<Vec<i32>>) -> Vec<Edge> {
    let mut edges: Vec<Edge> = vec![];
    for (i, pt1) in points.iter().enumerate() {
        for (j, pt2) in points[i + 1..].iter().enumerate() {
            edges.push(Edge::new(pt1, pt2, i, j + i + 1));
        }
    }
    edges.sort_by(|e1, e2| e1.weight.cmp(&e2.weight));
    edges
}

pub fn union(root: &mut Vec<usize>, ranks: &mut Vec<usize>, x: usize, y: usize) {
    let root_x = find(root, x);
    let root_y = find(root, y);

    if root_x != root_y {
        if ranks[root_x] > ranks[root_y] {
            root[root_y] = root_x;
        } else {
            root[root_x] = root_y;
            if ranks[root_x] == ranks[root_y] {
                ranks[root_y] += 1;
            }
        }
    }
}

pub fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] != x {
        root[x] = find(root, root[x]);
    }
    root[x]
}

pub fn connected(root: &mut Vec<usize>, x: usize, y: usize) -> bool {
    find(root, x) == find(root, y)
}
