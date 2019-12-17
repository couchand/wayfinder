use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::core::RouteConfig;
use crate::gen::codegen;
use crate::parse;
use crate::parse::errors::show_errors;

pub fn build<P: AsRef<Path>, W: Write>(src: &P, dest: &mut W) {
    let path = src.as_ref();

    let contents = std::fs::read_to_string(path.clone()).expect("load routes");

    match parse::route_config(&contents) {
        Ok(config) => match codegen(dest, &config.1) {
            Ok(_) => {}
            Err(e) => {
                println!("cargo:warning=Wayfinder codegen error: {}", e);
            }
        },
        result => {
            println!("cargo:warning=Route config parse error in {:?}:", path);
            show_errors(&mut std::io::stdout(), &contents, result, "cargo:warning=");
        }
    }
}

// TODO: a better name!
pub fn build_from_config(config: RouteConfig) {
    // TODO: not unwrap
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("routes.rs");
    let mut out_file = File::create(&out_path).expect("create file");

    match codegen(&mut out_file, &config) {
        Ok(_) => {}
        Err(e) => {
            println!("cargo:warning=Wayfinder codegen error: {}", e);
        }
    }
}
