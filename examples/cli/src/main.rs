extern crate uuid;
extern crate wayfinder;

use wayfinder::Match;

include!(concat!(env!("OUT_DIR"), "/routes.rs"));

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("Usage: {} ROUTE_TO_TEST", args[0]);
        return;
    }

    let result = routes::match_route(&mut {args[1].chars()}, wayfinder::Method::Get);
    println!("Parsed: {:?}", result);

    match result {
        Err(e) => println!("Error: {}", e),
        Ok(Match::NotFound) => println!("No matching route."),
        Ok(Match::NotAllowed) => println!("Route does not support method."),
        Ok(Match::Route(p)) => println!("Route to {}", p.to_path()),
        Ok(Match::Redirect(p)) => println!("Redirect to {}", p.to_path()),
    }
}
