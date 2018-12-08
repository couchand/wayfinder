extern crate wayfinder;

use wayfinder::gen::codegen;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} ROUTE_FILE", args[0]);
        return;
    }

    let path = &args[1];
    let contents = std::fs::read_to_string(path).unwrap();

    match wayfinder::parse::route_config(&contents) {
        // TODO: not unwrap
        Ok(config) => codegen(&mut std::io::stdout(), &config.1).unwrap(),
        result => {
            println!("cargo:warning=Template parse error in {:?}:", path);
            wayfinder::errors::show_errors(
                &mut std::io::stdout(),
                &contents,
                result,
                "cargo:warning=",
            );
        },
    }
}
