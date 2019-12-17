use wayfinder::build::build;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} ROUTE_FILE", args[0]);
        return;
    }

    let path = &args[1];

    build(&path, &mut std::io::stdout());
}
