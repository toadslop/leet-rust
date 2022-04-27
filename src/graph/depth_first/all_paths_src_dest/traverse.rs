use super::{graph::Graph, solution::Color};

pub fn traverse(
    graph: &Graph,
    visited: &mut Vec<Color>,
    current: usize,
    destination: usize,
) -> bool {
    if visited[current] != Color::White {
        return visited[current] == Color::Black;
    }

    let vertexes = graph.get_vertexes(current).unwrap();

    if vertexes.is_empty() {
        return current == destination;
    }

    visited[current] = Color::Grey;

    for &vertex in vertexes.iter() {
        if !traverse(graph, visited, vertex, destination) {
            return false;
        }
    }

    visited[current] = Color::Black;

    true
}
