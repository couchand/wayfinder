use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use wayfinder_core::RouteConfig;
use wayfinder_gen::codegen;
use wayfinder_parse::errors::show_errors;

pub fn build<P: AsRef<Path>, W: Write>(src: &P, dest: &mut W) {
    let path = src.as_ref();

    let contents = std::fs::read_to_string(path.clone()).expect("load routes");

    match wayfinder_parse::route_config(&contents) {
        // TODO: not unwrap
        Ok(config) => codegen(dest, &config.1).unwrap(),
        result => {
            println!("cargo:warning=Route config parse error in {:?}:", path);
            show_errors(&mut std::io::stdout(), &contents, result, "cargo:warning=");
        }
    }
}

// TODO: a better name!
pub fn build_from_config(config: RouteConfig) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("routes.rs");
    let mut out_file = File::create(&out_path).expect("create file");

    codegen(&mut out_file, &config).unwrap();
}
