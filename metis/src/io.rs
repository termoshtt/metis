//! I/O for METIS file formats

use std::path::*;

/// Errors raised because METIS graph file is in invalid format.
#[derive(Debug, thiserror::Error)]
pub enum InvalidGraphFileError {
    #[error("{}:0 METIS graph file does not have valid header", filename.display())]
    NoHeader {
        /// Name of METIS graph file
        filename: PathBuf,
    },
    #[error("{}:{line} METIS graph file have invalid line", filename.display())]
    InvalidLine {
        /// Name of METIS graph file
        filename: PathBuf,
        /// Where the invalid line is found
        line: usize,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn unweighted_graph() {
        // graph in Figure 2.(a)
        let _graph = r#"
            7 11
            5 3 2
            1 3 4
            5 4 2 1
            2 3 6 7
            1 3 6
            5 4 7
            6 4
        "#;
        unimplemented!()
    }

    #[test]
    fn weighted_graph_weights_on_edges() {
        // graph in Figure 2.(b)
        let _graph = r#"
            7 11 001
            5 1 3 2 2 1
            1 1 3 2 4 1
            5 3 4 2 2 2 1 2
            2 1 3 2 6 2 7 5
            1 1 3 3 6 2
            5 2 4 2 7 6
            6 6 4 5
        "#;
        unimplemented!()
    }

    #[test]
    fn weighted_graph_weights_both_on_vertices_and_edges() {
        // graph in Figure 2.(c)
        let _graph = r#"
            7 11 011
            4 5 1 3 2 2 1
            2 1 1 3 2 4 1
            5 5 3 4 2 2 2 1 2
            3 2 1 3 2 6 2 7 5
            1 1 1 3 3 6 2
            6 5 2 4 2 7 6
            2 6 6 4 5
        "#;
        unimplemented!()
    }

    #[test]
    fn multi_constraint_graph() {
        // graph in Figure 2.(d)
        let _graph = r#"
            7 11 010 3
            1 2 0 5 3 2
            0 2 2 1 3 4
            4 1 1 5 4 2 1
            2 2 3 2 3 6 7
            1 1 1 1 3 6
            2 2 1 5 4 7
            1 2 1 6 4
        "#;
        unimplemented!()
    }
}
