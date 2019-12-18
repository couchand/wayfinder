use wayfinder::build::Builder;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} ROUTE_FILE", args[0]);
        return;
    }

    let path = &args[1];

    Builder::new()
        .input_file(path)
        .output_stdout()
        .build();
}
