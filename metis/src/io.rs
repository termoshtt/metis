//! I/O for METIS file formats

use std::path::*;

#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum LineError {
    #[error("vertex size `s` in manual is missing")]
    VertexSizeMissing,

    #[error("edge weight does not exists")]
    EdgeWeightMissing,

    #[error("Vertex is out-of-range: {index} > {num_vertices}")]
    VertexOutOfRange { index: i32, num_vertices: i32 },

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

/// Errors raised because METIS graph file is in invalid format.
#[derive(Debug, thiserror::Error)]
pub enum InvalidGraphFileError {
    #[error("{}:0 METIS graph file does not have valid header", filename.display())]
    NoHeader {
        /// Name of METIS graph file.
        /// This may be empty if graph is loaded from string.
        filename: PathBuf,
    },

    #[error("{}:{line_position} METIS graph file have invalid line: {error:?}", filename.display())]
    InvalidLine {
        /// Error type
        error: LineError,
        /// Name of METIS graph file
        filename: PathBuf,
        /// Where the invalid line is found
        line_position: usize,
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
        let mut split_iter = line.trim().split_whitespace();
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

#[derive(Debug)]
struct Line {
    /// `s` in manual
    /// None if Header.has_vertex_size is false
    vertex_size: Option<i32>,
    /// `w_1`, `w_2`, ... in manual
    /// None if Header.has_vertex_weight is false
    vertex_weights: Option<Vec<f32>>,
    /// `v1`, ... in manual
    vertices: Vec<i32>,
    /// `e1`, ... in manual
    /// None if Header.has_edge_weight is false
    edge_weights: Option<Vec<f32>>,
}

impl Line {
    fn parse(header: &Header, line: &str) -> Result<Self, LineError> {
        let mut nums = line.trim().split_whitespace();
        let vertex_size = if header.fmt.has_vertex_size {
            let s = nums.next().ok_or(LineError::VertexSizeMissing)?;
            let s: i32 = s.parse()?;
            Some(s)
        } else {
            None
        };
        let vertex_weights = if header.fmt.has_vertex_weight {
            let ws = nums
                .by_ref()
                .take(header.num_weights)
                .map(|num| num.parse())
                .collect::<Result<Vec<f32>, _>>()?;
            Some(ws)
        } else {
            None
        };
        let (vertices, edge_weights) = if header.fmt.has_edge_weight {
            let mut vs = Vec::new();
            let mut es = Vec::new();
            loop {
                let v = match nums.next() {
                    Some(v) => v.parse::<i32>()?,
                    None => break,
                };
                vs.push(v);
                let e = nums
                    .next()
                    .ok_or(LineError::EdgeWeightMissing)?
                    .parse::<f32>()?;
                es.push(e);
            }
            (vs, Some(es))
        } else {
            let mut vs = Vec::new();
            loop {
                let v = match nums.next() {
                    Some(v) => v.parse::<i32>()?,
                    None => break,
                };
                vs.push(v);
            }
            (vs, None)
        };
        for &index in &vertices {
            let num_vertices = header.num_vertices as i32;
            if index > num_vertices {
                return Err(LineError::VertexOutOfRange {
                    index,
                    num_vertices,
                });
            }
        }
        Ok(Self {
            vertex_size,
            vertex_weights,
            vertices,
            edge_weights,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod format {
        use super::*;
        #[test]
        fn new() {
            let fmt = Format::new("011");
            dbg!(&fmt);
            assert!(!fmt.has_vertex_size);
            assert!(fmt.has_vertex_weight);
            assert!(fmt.has_edge_weight);
        }

        #[should_panic]
        #[test]
        fn new_invalid1() {
            let _fmt = Format::new("0111");
        }

        #[should_panic]
        #[test]
        fn new_invalid2() {
            let _fmt = Format::new("012");
        }

        #[should_panic]
        #[test]
        fn new_invalid3() {
            let _fmt = Format::new("01");
        }
    }

    mod header {
        use super::*;
        #[test]
        fn parse_success() {
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

            // multi-space
            let header = Header::parse("", "10   34 	 011 3");
            assert_eq!(header.num_vertices, 10);
            assert_eq!(header.num_edges, 34);
            assert_eq!(header.fmt, Format::new("011"));
            assert_eq!(header.num_weights, 3);
        }

        #[should_panic]
        #[test]
        fn parse_fail_negative() {
            let _ = Header::parse("", "10 -34");
        }
    }

    mod line {
        use super::*;

        #[test]
        fn parse_default() {
            let header = Header::parse("", "100 100");
            let line = Line::parse(&header, "1 10 30").unwrap();
            assert!(line.vertex_size.is_none());
            assert!(line.vertex_weights.is_none());
            assert!(line.edge_weights.is_none());
            assert_eq!(line.vertices, vec![1, 10, 30]);

            // multi-space
            let line = Line::parse(&header, "1  10 	 30").unwrap();
            assert!(line.vertex_size.is_none());
            assert!(line.vertex_weights.is_none());
            assert!(line.edge_weights.is_none());
            assert_eq!(line.vertices, vec![1, 10, 30]);
        }

        #[test]
        fn parse_edge_weight() {
            let header = Header::parse("", "100 100 001");
            let line = Line::parse(&header, "1 12.34 10 5678 30 -999").unwrap();
            assert!(line.vertex_size.is_none());
            assert!(line.vertex_weights.is_none());
            assert_eq!(line.vertices, vec![1, 10, 30]);
            assert_eq!(line.edge_weights.unwrap(), vec![12.34, 5678.0, -999.0]);
        }

        #[test]
        fn parse_vertex_weight() {
            let header = Header::parse("", "100 100 010 3");
            let line = Line::parse(&header, "0.1 -3.0 10 1 10 30").unwrap();
            assert!(line.vertex_size.is_none());
            assert!(line.edge_weights.is_none());
            assert_eq!(line.vertices, vec![1, 10, 30]);
            assert_eq!(line.vertex_weights.unwrap(), vec![0.1, -3.0, 10.0]);
        }

        #[test]
        fn vertex_out_of_range() {
            let header = Header::parse("", "10 20"); // num_vertices = 10
            let result = Line::parse(&header, "100 200"); // index = 100 is too large
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                LineError::VertexOutOfRange {
                    index: 100,
                    num_vertices: 10
                }
            );
        }
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
        "#
        .trim();
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
        "#
        .trim();
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
        "#
        .trim();
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
        "#
        .trim();
        unimplemented!()
    }
}
