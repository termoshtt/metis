use std::{env, path::*};

fn main() {
    let metis_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("metis-5.1.0");
    let dst = cmake::Config::new(&metis_dir)
        .define("GKLIB_PATH", metis_dir.join("GKlib"))
        .build();
    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=metis");
}
