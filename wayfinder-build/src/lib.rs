use std::io::Write;
use std::path::Path;

use wayfinder_parse::errors::show_errors;
use wayfinder_gen::codegen;

pub fn build<P: AsRef<Path>, W: Write>(src: &P, dest: &mut W) {
    let path = src.as_ref();

    let contents = std::fs::read_to_string(path.clone()).expect("load routes");

    match wayfinder_parse::route_config(&contents) {
        // TODO: not unwrap
        Ok(config) => codegen(dest, &config.1).unwrap(),
        result => {
            println!("cargo:warning=Route config parse error in {:?}:", path);
            show_errors(
                &mut std::io::stdout(),
                &contents,
                result,
                "cargo:warning=",
            );
        },
    }
}
