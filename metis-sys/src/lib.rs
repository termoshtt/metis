//! *-sys crate for METIS
//!
//! Examples
//! ---------
//!
//! ```
//! use std::ptr::null_mut;
//! use metis_sys::*;
//!
//! // Graph data in Figure 3 (b) in manual
//! const NUM_VERTICES: i32 = 15;
//! const NUM_EDGES: i32 = 22;
//! let mut xadj: [i32; NUM_VERTICES as usize + 1]
//!     = [0, 2, 5, 8, 11, 13, 16, 20, 24, 28, 31, 33, 36, 39, 42, 44];
//! let mut adjncy: [i32; 2 * NUM_EDGES as usize]
//!     = [1, 5, 0, 2, 6, 1, 3, 7, 2, 4, 8, 3, 9, 0, 6, 10, 1, 5, 7, 11, 2, 6, 8,
//!        12, 3, 7, 9, 13, 4, 8, 14, 5, 11, 6, 10, 12, 7, 11, 13, 8, 12, 14, 9, 13];
//!
//! // Partition configure
//! const NUM_WEIGHTS: i32 = 1;
//! const NUM_PARTS: i32 = 4; // split into 4 parts
//!
//! // Outputs values
//! let mut objval = 0;
//! let mut part: [i32; NUM_VERTICES as usize] = Default::default();
//!
//! // Call API
//! let ret = unsafe {
//!     METIS_PartGraphKway(
//!         &NUM_VERTICES as *const i32 as *mut i32,
//!         &NUM_WEIGHTS as *const i32 as *mut i32,
//!         xadj.as_mut_ptr(),
//!         adjncy.as_mut_ptr(),
//!         null_mut(), // vwgt   = the weights of the vertices
//!         null_mut(), // vsize  = the size of the vertices
//!         null_mut(), // adjwgt = the weights of the edges
//!         &NUM_PARTS as *const i32 as *mut i32,
//!         null_mut(), // tpwgts = The desired weights for each partition
//!         null_mut(), // ubvec  = The allowed weights
//!         null_mut(), // options
//!         &mut objval,
//!         part.as_mut_ptr()
//!     )
//! };
//! assert_eq!(ret, rstatus_et::METIS_OK as i32);
//! dbg!(objval);
//! dbg!(part);
//! ```

#![allow(non_camel_case_types)]

#[cfg(feature = "source")]
extern crate metis_src as _src;

#[cfg_attr(feature = "system", link(name = "metis"))]
extern "C" {}

use num_derive::*;

include!("metis.rs");
