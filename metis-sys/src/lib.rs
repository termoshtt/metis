#![allow(non_camel_case_types)]

#[cfg(feature = "source")]
extern crate metis_src as _src;

#[cfg_attr(feature = "system", link(name = "metis"))]
extern "C" {}

use num_derive::*;

include!("metis.rs");
