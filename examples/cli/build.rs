extern crate wayfinder_build;

use std::env;
use std::fs::File;
use std::path::PathBuf;

use wayfinder_build::build;

fn main() {
    let in_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let in_path = in_dir.join("..").join("..").join("example.routes");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("routes.rs");
    let mut out_file = File::create(&out_path).expect("create file");

    build(&in_path, &mut out_file);
}
