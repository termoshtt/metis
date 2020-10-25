//! Graph structures

use crate::{error::*, io::graph::*};
use std::{mem::MaybeUninit, ptr::null_mut};

/// uncompressed graph
#[derive(Debug, Clone)]
pub struct UndirectedGraph {
    vertex_size: usize,
    edges: Vec<(i32, i32)>,
}

impl FromMetisGraphFormat for UndirectedGraph {
    fn from_metis_graph_iter(
        header: &Header,
        lines: impl Iterator<Item = std::result::Result<Line, LineError>>,
    ) -> std::result::Result<Self, GraphFileError> {
        let mut edges = Vec::with_capacity(header.num_edges);
        for line in lines {
            let line = line?;
            let from_index = line.from_index;
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

/// Compressed sparse row (CSR) format for general (non-symmetric) graph matrix
#[derive(Debug, Clone, PartialEq)]
pub struct CSRGraph {
    /// `adjncy` in METIS manual
    column_indices: Vec<i32>,
    /// `xadj` in METIS manual
    num_elements_in_row_cumsum: Vec<i32>,
}

impl CSRGraph {
    pub fn num_vertices(&self) -> usize {
        self.num_elements_in_row_cumsum.len() - 1
    }

    pub fn num_edges(&self) -> usize {
        self.column_indices.len() / 2
    }

    pub fn part_kway(&mut self, num_partitions: i32) -> Result<(i32, Vec<i32>)> {
        let num_vertices = self.num_vertices() as i32;
        let num_weights = 1_i32;
        let mut objval = MaybeUninit::uninit();
        let mut part = Vec::with_capacity(self.num_vertices());
        unsafe {
            metis_sys::METIS_PartGraphKway(
                &num_vertices as *const i32 as *mut i32,
                &num_weights as *const i32 as *mut i32,
                self.num_elements_in_row_cumsum.as_mut_ptr(),
                self.column_indices.as_mut_ptr(),
                null_mut(), // vwgt   = the weights of the vertices
                null_mut(), // vsize  = the size of the vertices
                null_mut(), // adjwgt = the weights of the edges
                &num_partitions as *const i32 as *mut i32,
                null_mut(), // tpwgts = The desired weights for each partition
                null_mut(), // ubvec  = The allowed weights
                null_mut(), // options
                objval.as_mut_ptr(),
                part.as_mut_ptr(),
            )
        }
        .check("METIS_PartGraphKway")?;
        Ok((unsafe { objval.assume_init() }, part))
    }
}

impl FromMetisGraphFormat for CSRGraph {
    fn from_metis_graph_iter(
        header: &Header,
        lines: impl Iterator<Item = std::result::Result<Line, LineError>>,
    ) -> std::result::Result<Self, GraphFileError> {
        let mut graph = CSRGraph {
            column_indices: Vec::with_capacity(2 * header.num_edges),
            num_elements_in_row_cumsum: Vec::with_capacity(header.num_vertices + 1),
        };
        let mut num_elements = 0;
        graph.num_elements_in_row_cumsum.push(num_elements);
        for line in lines {
            let line = line?;
            num_elements += line.vertices.len() as i32;
            for vertex in line.vertices {
                graph.column_indices.push(vertex);
            }
            graph.num_elements_in_row_cumsum.push(num_elements);
        }
        if graph.num_vertices() != header.num_vertices {
            return Err(GraphFileError::VertexSizeMissing {
                actual: graph.num_vertices(),
                header: header.num_vertices,
            });
        }
        if graph.column_indices.len() % 2 != 0 {
            return Err(GraphFileError::NonSymmetric);
        }
        if graph.num_edges() != header.num_edges {
            return Err(GraphFileError::EdgeSizeMissmatch {
                actual: graph.num_edges(),
                header: header.num_edges,
            });
        }
        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod undirected {
        use super::*;
        #[test]
        fn manual_2a() {
            let _graph = UndirectedGraph::from_metis_graph_str(examples::MANUAL_2A).unwrap();
        }
        #[test]
        fn manual_2b() {
            let _graph = UndirectedGraph::from_metis_graph_str(examples::MANUAL_2B).unwrap();
        }
        #[test]
        fn manual_2c() {
            let _graph = UndirectedGraph::from_metis_graph_str(examples::MANUAL_2C).unwrap();
        }
        #[test]
        fn manual_2d() {
            let _graph = UndirectedGraph::from_metis_graph_str(examples::MANUAL_2D).unwrap();
        }
    }

    mod csr {
        use super::*;
        #[test]
        fn manual_2a() {
            let _graph = CSRGraph::from_metis_graph_str(examples::MANUAL_2A).unwrap();
        }
        #[test]
        fn manual_2b() {
            let _graph = CSRGraph::from_metis_graph_str(examples::MANUAL_2B).unwrap();
        }
        #[test]
        fn manual_2c() {
            let _graph = CSRGraph::from_metis_graph_str(examples::MANUAL_2C).unwrap();
        }
        #[test]
        fn manual_2d() {
            let _graph = CSRGraph::from_metis_graph_str(examples::MANUAL_2D).unwrap();
        }

        #[test]
        fn grid() {
            let graph = CSRGraph::from_metis_graph_str(examples::MANUAL_3A).unwrap();
            // Copy from Figure 3 (b) in the manual
            let ans = CSRGraph {
                column_indices: vec![
                    1, 5, 0, 2, 6, 1, 3, 7, 2, 4, 8, 3, 9, 0, 6, 10, 1, 5, 7, 11, 2, 6, 8, 12, 3,
                    7, 9, 13, 4, 8, 14, 5, 11, 6, 10, 12, 7, 11, 13, 8, 12, 14, 9, 13,
                ],
                num_elements_in_row_cumsum: vec![
                    0, 2, 5, 8, 11, 13, 16, 20, 24, 28, 31, 33, 36, 39, 42, 44,
                ],
            };
            assert_eq!(graph, ans);
        }
    }
}
