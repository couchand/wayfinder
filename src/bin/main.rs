extern crate wayfinder;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} ROUTE_FILE", args[0]);
    }

    let path = &args[1];
    let contents = std::fs::read_to_string(path).unwrap();

    match wayfinder::parse::route_config(&contents) {
        Ok(config) => print!("{}", config.1.stringify()),
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
