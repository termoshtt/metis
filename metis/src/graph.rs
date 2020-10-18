use crate::io::graph::*;

/// uncompressed graph
#[derive(Debug, Clone)]
pub struct UndirectedGraph {
    vertex_size: usize,
    edges: Vec<(i32, i32)>,
}

impl FromMetisGraphFormat for UndirectedGraph {
    fn from_metis_graph_iter(
        header: &Header,
        lines: impl Iterator<Item = Result<Line, LineError>>,
    ) -> Result<Self, GraphFileError> {
        let mut edges = Vec::new();
        for line in lines {
            let line = line?;
            let from_index = line.position;
            for to_index in line.vertices {
                if from_index < to_index {
                    edges.push((from_index, to_index));
                }
            }
        }

        // Check edge size
        if edges.len() != header.num_edges {
            return Err(GraphFileError::EdgeSizeMissmatch {
                actual: edges.len(),
                header: header.num_edges,
            });
        }
        Ok(UndirectedGraph {
            vertex_size: header.num_vertices as usize,
            edges,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unweighted_graph() {
        // graph in Figure 2.(a)
        let input = r#"
            7 11
            5 3 2
            1 3 4
            5 4 2 1
            2 3 6 7
            1 3 6
            5 4 7
            6 4
        "#;
        let _graph = UndirectedGraph::from_metis_graph_str(input).unwrap();
    }

    #[test]
    fn weighted_graph_weights_on_edges() {
        // graph in Figure 2.(b)
        let input = r#"
            7 11 001
            5 1 3 2 2 1
            1 1 3 2 4 1
            5 3 4 2 2 2 1 2
            2 1 3 2 6 2 7 5
            1 1 3 3 6 2
            5 2 4 2 7 6
            6 6 4 5
        "#;
        let _graph = UndirectedGraph::from_metis_graph_str(input).unwrap();
    }

    #[test]
    fn weighted_graph_weights_both_on_vertices_and_edges() {
        // graph in Figure 2.(c)
        let input = r#"
            7 11 011
            4 5 1 3 2 2 1
            2 1 1 3 2 4 1
            5 5 3 4 2 2 2 1 2
            3 2 1 3 2 6 2 7 5
            1 1 1 3 3 6 2
            6 5 2 4 2 7 6
            2 6 6 4 5
        "#;
        let _graph = UndirectedGraph::from_metis_graph_str(input).unwrap();
    }

    #[test]
    fn multi_constraint_graph() {
        // graph in Figure 2.(d)
        let input = r#"
            7 11 010 3
            1 2 0 5 3 2
            0 2 2 1 3 4
            4 1 1 5 4 2 1
            2 2 3 2 3 6 7
            1 1 1 1 3 6
            2 2 1 5 4 7
            1 2 1 6 4
        "#;
        let _graph = UndirectedGraph::from_metis_graph_str(input).unwrap();
    }
}
