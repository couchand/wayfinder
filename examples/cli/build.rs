extern crate wayfinder;

use std::env;
use std::fs::File;
use std::path::PathBuf;

use wayfinder::gen::codegen;

fn main() {
    let in_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let in_path = in_dir.join("..").join("..").join("example.routes");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("routes.rs");
    let mut out_file = File::create(&out_path).expect("create file");

    let contents = std::fs::read_to_string(in_path.clone()).expect("load routes");

    match wayfinder::parse::route_config(&contents) {
        // TODO: not unwrap
        Ok(config) => codegen(&mut out_file, &config.1).unwrap(),
        result => {
            println!("cargo:warning=Route config parse error in {:?}:", in_path);
            wayfinder::errors::show_errors(
                &mut std::io::stdout(),
                &contents,
                result,
                "cargo:warning=",
            );
        },
    }
}
