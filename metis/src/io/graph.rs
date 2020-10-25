//! I/O for METIS Graph formats

use std::{
    fs,
    io::{self, BufRead},
    path::*,
    str::FromStr,
};

/// Example graphs
pub mod examples {
    /// graph in Figure 2 (a) of the manual
    pub const MANUAL_2A: &str = r#"
        7 11
        5 3 2
        1 3 4
        5 4 2 1
        2 3 6 7
        1 3 6
        5 4 7
        6 4
    "#;

    /// graph in Figure 2 (b) of the manual
    pub const MANUAL_2B: &str = r#"
        7 11 001
        5 1 3 2 2 1
        1 1 3 2 4 1
        5 3 4 2 2 2 1 2
        2 1 3 2 6 2 7 5
        1 1 3 3 6 2
        5 2 4 2 7 6
        6 6 4 5
    "#;

    /// graph in Figure 2 (c) of the manual
    pub const MANUAL_2C: &str = r#"
        7 11 011
        4 5 1 3 2 2 1
        2 1 1 3 2 4 1
        5 5 3 4 2 2 2 1 2
        3 2 1 3 2 6 2 7 5
        1 1 1 3 3 6 2
        6 5 2 4 2 7 6
        2 6 6 4 5
    "#;

    /// graph in Figure 2 (d) of the manual
    pub const MANUAL_2D: &str = r#"
        7 11 010 3
        1 2 0 5 3 2
        0 2 2 1 3 4
        4 1 1 5 4 2 1
        2 2 3 2 3 6 7
        1 1 1 1 3 6
        2 2 1 5 4 7
        1 2 1 6 4
    "#;

    /// graph in Figure 3 (a) of the manual
    pub const MANUAL_3A: &str = r#"
        15 22
        1 5
        0 2 6
        1 3 7
        2 4 8
        3 9
        0 6 10
        1 5 7 11
        2 6 8 12
        3 7 9 13
        4 8 14
        5 11
        6 10 12
        7 11 13
        8 12 14
        9 13
    "#;
}

/// Constructable from METIS Graph format
pub trait FromMetisGraphFormat: Sized {
    fn from_metis_graph_iter(
        header: &Header,
        lines: impl Iterator<Item = Result<Line, LineError>>,
    ) -> Result<Self, GraphFileError>;

    /// Read METIS graph file as a string (assumes to be small)
    fn from_metis_graph_str(input: &str) -> Result<Self, GraphFileError> {
        // This default impl assumes `input` is not too large
        Self::from_metis_graph_lines(input.trim().lines().map(|line| line.to_string()))
    }

    /// Read METIS graph file
    fn from_metis_graph(path: impl AsRef<Path>) -> Result<Self, GraphFileError> {
        let mut f = fs::File::open(path.as_ref())?;
        let buf = io::BufReader::new(&mut f);
        Self::from_metis_graph_lines(buf.lines().map(|line| line.unwrap()))
    }

    // common default implementations
    #[doc(hidden)]
    fn from_metis_graph_lines(
        mut lines: impl Iterator<Item = String>,
    ) -> Result<Self, GraphFileError> {
        let header = Header::from_str(
            &lines
                .next()
                .ok_or(GraphFileError::InvalidHeader(HeaderError::Empty))?,
        )?;
        let lines = lines.enumerate().map(|(from_index, line)| {
            let from_index = from_index as i32 + 1;
            Line::parse(&header, from_index, &line)
        });
        Self::from_metis_graph_iter(&header, lines)
    }
}

/// Error for invalid line
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

/// Error for invalid header line
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum HeaderError {
    #[error("Header is empty")]
    Empty,

    #[error("Header does not have edge size")]
    EdgeSizeMissing,

    #[error("Format spec in header is invalid: {fmt}")]
    InvalidFormat { fmt: String },

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

/// Errors raised because METIS graph file is in invalid format.
#[derive(Debug, thiserror::Error)]
pub enum GraphFileError {
    #[error(transparent)]
    InvalidHeader(#[from] HeaderError),

    #[error(transparent)]
    InvalidLine(#[from] LineError),

    #[error("Vertex size mismatch: actual({actual}) != header({header})")]
    VertexSizeMissing { actual: usize, header: usize },

    #[error("Edge size mismatch: actual({actual}) != header({header})")]
    EdgeSizeMissmatch { actual: usize, header: usize },

    #[error("METIS Graph format is assumed to be symmetric, but has odd number edges.")]
    NonSymmetric,

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

/// Graph file format specification in the header
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Format {
    pub has_vertex_size: bool,
    pub has_vertex_weight: bool,
    pub has_edge_weight: bool,
}

impl FromStr for Format {
    type Err = HeaderError;
    fn from_str(fmt: &str) -> Result<Self, HeaderError> {
        if fmt.len() != 3 {
            return Err(HeaderError::InvalidFormat { fmt: fmt.into() });
        }

        let byte2bool = |byte| match byte {
            b'1' => Ok(true),
            b'0' => Ok(false),
            _ => Err(HeaderError::InvalidFormat { fmt: fmt.into() }),
        };
        let bytes = fmt.as_bytes();
        Ok(Format {
            has_vertex_size: byte2bool(bytes[0])?,
            has_vertex_weight: byte2bool(bytes[1])?,
            has_edge_weight: byte2bool(bytes[2])?,
        })
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
pub struct Header {
    /// Number of vertices
    pub num_vertices: usize,
    /// Number of edges
    pub num_edges: usize,
    pub fmt: Format,
    /// Number of vertex weights associated with each vertex of the graph
    pub num_weights: usize,
}

impl FromStr for Header {
    type Err = HeaderError;
    fn from_str(line: &str) -> Result<Self, HeaderError> {
        let mut split_iter = line.trim().split_whitespace();
        let num_vertices = split_iter.next().ok_or(HeaderError::Empty)?.parse()?;
        let num_edges = split_iter
            .next()
            .ok_or(HeaderError::EdgeSizeMissing)?
            .parse()?;
        let fmt = match split_iter.next() {
            Some(fmt) => Format::from_str(fmt)?,
            None => Format::default(),
        };
        // If this parameter is omitted,
        // then the vertices of the graph are assumed to have a single weight
        let num_weights = match split_iter.next() {
            Some(ncon) => ncon.parse()?,
            None => 1,
        };
        Ok(Header {
            num_vertices,
            num_edges,
            fmt,
            num_weights,
        })
    }
}

/// Parsed line in METIS graph format
#[derive(Debug)]
pub struct Line {
    /// Corresponding vertex index
    pub from_index: i32,
    /// `s` in manual
    /// None if Header.has_vertex_size is false
    pub vertex_size: Option<i32>,
    /// `w_1`, `w_2`, ... in manual
    /// None if Header.has_vertex_weight is false
    pub vertex_weights: Option<Vec<f32>>,
    /// `v1`, ... in manual
    pub vertices: Vec<i32>,
    /// `e1`, ... in manual
    /// None if Header.has_edge_weight is false
    pub edge_weights: Option<Vec<f32>>,
}

impl Line {
    pub fn parse(header: &Header, from_index: i32, line: &str) -> Result<Self, LineError> {
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
            while let Some(v) = nums.next() {
                vs.push(v.parse()?);
                let e = nums.next().ok_or(LineError::EdgeWeightMissing)?.parse()?;
                es.push(e);
            }
            (vs, Some(es))
        } else {
            let vs = nums.map(|v| v.parse()).collect::<Result<Vec<i32>, _>>()?;
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
            from_index,
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
            let fmt = Format::from_str("011").unwrap();
            assert!(!fmt.has_vertex_size);
            assert!(fmt.has_vertex_weight);
            assert!(fmt.has_edge_weight);
        }

        #[should_panic]
        #[test]
        fn new_invalid1() {
            let _fmt = Format::from_str("0111").unwrap();
        }

        #[should_panic]
        #[test]
        fn new_invalid2() {
            let _fmt = Format::from_str("012").unwrap();
        }

        #[should_panic]
        #[test]
        fn new_invalid3() {
            let _fmt = Format::from_str("01").unwrap();
        }
    }

    mod header {
        use super::*;
        #[test]
        fn parse_success() {
            let header = Header::from_str("10 34").unwrap();
            assert_eq!(header.num_vertices, 10);
            assert_eq!(header.num_edges, 34);
            assert_eq!(header.fmt, Format::default());
            assert_eq!(header.num_weights, 1);

            let header = Header::from_str("10 34 011").unwrap();
            assert_eq!(header.num_vertices, 10);
            assert_eq!(header.num_edges, 34);
            assert_eq!(header.fmt, Format::from_str("011").unwrap());
            assert_eq!(header.num_weights, 1);

            let header = Header::from_str("10 34 011 3").unwrap();
            assert_eq!(header.num_vertices, 10);
            assert_eq!(header.num_edges, 34);
            assert_eq!(header.fmt, Format::from_str("011").unwrap());
            assert_eq!(header.num_weights, 3);

            // multi-space
            let header = Header::from_str("10   34 	 011 3").unwrap();
            assert_eq!(header.num_vertices, 10);
            assert_eq!(header.num_edges, 34);
            assert_eq!(header.fmt, Format::from_str("011").unwrap());
            assert_eq!(header.num_weights, 3);
        }

        #[should_panic]
        #[test]
        fn parse_fail_negative() {
            let _ = Header::from_str("10 -34").unwrap();
        }
    }

    mod line {
        use super::*;

        #[test]
        fn parse_default() {
            let header = Header::from_str("100 100").unwrap();
            let line = Line::parse(&header, 3, "1 10 30").unwrap();
            assert_eq!(line.from_index, 3);
            assert!(line.vertex_size.is_none());
            assert!(line.vertex_weights.is_none());
            assert!(line.edge_weights.is_none());
            assert_eq!(line.vertices, vec![1, 10, 30]);

            // multi-space
            let line = Line::parse(&header, 3, "1  10 	 30").unwrap();
            assert_eq!(line.from_index, 3);
            assert!(line.vertex_size.is_none());
            assert!(line.vertex_weights.is_none());
            assert!(line.edge_weights.is_none());
            assert_eq!(line.vertices, vec![1, 10, 30]);
        }

        #[test]
        fn parse_edge_weight() {
            let header = Header::from_str("100 100 001").unwrap();
            let line = Line::parse(&header, 3, "1 12.34 10 5678 30 -999").unwrap();
            assert_eq!(line.from_index, 3);
            assert!(line.vertex_size.is_none());
            assert!(line.vertex_weights.is_none());
            assert_eq!(line.vertices, vec![1, 10, 30]);
            assert_eq!(line.edge_weights.unwrap(), vec![12.34, 5678.0, -999.0]);
        }

        #[test]
        fn parse_vertex_weight() {
            let header = Header::from_str("100 100 010 3").unwrap();
            let line = Line::parse(&header, 3, "0.1 -3.0 10 1 10 30").unwrap();
            assert_eq!(line.from_index, 3);
            assert!(line.vertex_size.is_none());
            assert!(line.edge_weights.is_none());
            assert_eq!(line.vertices, vec![1, 10, 30]);
            assert_eq!(line.vertex_weights.unwrap(), vec![0.1, -3.0, 10.0]);
        }

        #[test]
        fn vertex_out_of_range() {
            let header = Header::from_str("10 20").unwrap(); // num_vertices = 10
            let result = Line::parse(&header, 3, "100 200"); // index = 100 is too large
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
}
