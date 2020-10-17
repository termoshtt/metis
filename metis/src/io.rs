//! I/O for METIS file formats

use std::path::*;

/// Errors raised because METIS graph file is in invalid format.
#[derive(Debug, thiserror::Error)]
pub enum InvalidGraphFileError {
    #[error("{}:0 METIS graph file does not have valid header", filename.display())]
    NoHeader {
        /// Name of METIS graph file.
        /// This may be empty if graph is loaded from string.
        filename: PathBuf,
    },
    #[error("{}:{line} METIS graph file have invalid line", filename.display())]
    InvalidLine {
        /// Name of METIS graph file
        filename: PathBuf,
        /// Where the invalid line is found
        line: usize,
    },

    #[error("METIS graph file not found")]
    FileNotFound(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Format {
    has_vertex_size: bool,
    has_vertex_weight: bool,
    has_edge_weight: bool,
}

impl Format {
    fn new(fmt: &str) -> Self {
        assert_eq!(fmt.len(), 3);
        let byte2bool = |byte| match byte {
            b'1' => true,
            b'0' => false,
            _ => panic!("Invalid format specification in Graph file header: {}", fmt),
        };
        let bytes = fmt.as_bytes();
        Format {
            has_vertex_size: byte2bool(bytes[0]),
            has_vertex_weight: byte2bool(bytes[1]),
            has_edge_weight: byte2bool(bytes[2]),
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format {
            has_edge_weight: false,
            has_vertex_weight: false,
            has_vertex_size: false,
        }
    }
}

/// Header of METIS Graph file
#[derive(Debug, Clone, PartialEq)]
struct Header {
    filename: PathBuf,
    /// Number of vertices
    num_vertices: usize,
    /// Number of edges
    num_edges: usize,
    fmt: Format,
    /// Number of vertex weights associated with each vertex of the graph
    num_weights: usize,
}

impl Header {
    fn parse<P: AsRef<Path>>(filename: P, line: &str) -> Self {
        dbg!(line);
        let mut split_iter = line.trim().split(" ");
        let num_vertices = split_iter
            .next()
            .expect("Graph file header does not contain the number of vertices")
            .parse()
            .expect("Failed to parse number of vertices");
        let num_edges = split_iter
            .next()
            .expect("Graph file header does not contain the number of edges")
            .parse()
            .expect("Failed to parse number of edges");
        let fmt = match split_iter.next() {
            Some(fmt) => Format::new(fmt),
            None => Format::default(),
        };
        // If this parameter is omitted,
        // then the vertices of the graph are assumed to have a single weight
        let num_weights = match split_iter.next() {
            Some(ncon) => ncon.parse().expect("Failed to parse num_weights"),
            None => 1,
        };
        Header {
            filename: filename.as_ref().to_owned(),
            num_vertices,
            num_edges,
            fmt,
            num_weights,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_new() {
        let fmt = Format::new("011");
        dbg!(&fmt);
        assert!(!fmt.has_vertex_size);
        assert!(fmt.has_vertex_weight);
        assert!(fmt.has_edge_weight);
    }

    #[should_panic]
    #[test]
    fn format_new_invalid1() {
        let _fmt = Format::new("0111");
    }

    #[should_panic]
    #[test]
    fn format_new_invalid2() {
        let _fmt = Format::new("012");
    }

    #[should_panic]
    #[test]
    fn format_new_invalid3() {
        let _fmt = Format::new("01");
    }

    #[test]
    fn parse_header_success() {
        let header = Header::parse("", "10 34");
        assert_eq!(header.num_vertices, 10);
        assert_eq!(header.num_edges, 34);
        assert_eq!(header.fmt, Format::default());
        assert_eq!(header.num_weights, 1);

        let header = Header::parse("", "10 34 011");
        assert_eq!(header.num_vertices, 10);
        assert_eq!(header.num_edges, 34);
        assert_eq!(header.fmt, Format::new("011"));
        assert_eq!(header.num_weights, 1);

        let header = Header::parse("", "10 34 011 3");
        assert_eq!(header.num_vertices, 10);
        assert_eq!(header.num_edges, 34);
        assert_eq!(header.fmt, Format::new("011"));
        assert_eq!(header.num_weights, 3);
    }

    #[should_panic]
    #[test]
    fn parse_header_fail_negative() {
        let _ = Header::parse("", "10 -34");
    }

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
