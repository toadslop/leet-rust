#[derive(Debug)]
pub struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn with_capacity(n: i32) -> Self {
        Graph {
            edges: vec![vec![]; n as usize],
        }
    }

    pub fn add_edges(&mut self, edges: Vec<Vec<i32>>) {
        for edge in edges.iter() {
            self.add(edge)
        }
    }

    fn add(&mut self, edge: &Vec<i32>) {
        if let Some(vertex_list) = self.edges.get_mut(edge[0] as usize) {
            vertex_list.push(edge[1] as usize)
        } else {
            self.edges.insert(edge[0] as usize, vec![edge[1] as usize])
        }
    }

    pub fn get_vertexes(&self, x: usize) -> Option<&Vec<usize>> {
        self.edges.get(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_has_specifified_capacity() -> () {
        let graph = Graph::with_capacity(5);
        assert_eq!(graph.edges.capacity(), 5);
    }

    #[test]
    fn first_edge_added_has_correct_value() -> () {
        let mut graph = Graph::with_capacity(5);
        graph.add(&vec![3, 2]);
        assert_eq!(graph.edges[3][0], 2);
    }

    #[test]
    fn second_edge_added_has_correct_value() -> () {
        let mut graph = Graph::with_capacity(5);
        graph.add(&vec![3, 2]);
        graph.add(&vec![3, 4]);
        assert_eq!(graph.edges[3][1], 4);
    }

    #[test]
    fn adding_vec_of_edges_has_correct_result() -> () {
        let mut graph = Graph::with_capacity(3);
        graph.add_edges(vec![vec![3, 2], vec![3, 4]]);
        assert_eq!(graph.edges[3][0], 2);
        assert_eq!(graph.edges[3][1], 4);
    }
}
