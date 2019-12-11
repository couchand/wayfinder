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

    let result = routes::match_route(&args[1], wayfinder::Method::Get);
    println!("Parsed: {:?}", result);

    match result {
        Err(e) => println!("Error: {}", e),
        Ok(Match::NotFound) => println!("No matching route."),
        Ok(Match::NotAllowed) => println!("Route does not support method."),
        Ok(Match::Route(p)) => println!("Route to {}", p.to_path()),
        Ok(Match::Redirect(p)) => println!("Redirect to {}", p.to_path()),
    }
}

#[cfg(test)]
mod tests {
    use super::routes;
    use wayfinder::Match;

    #[test]
    fn test_people_index() {
        let route = "/people";
        match routes::match_route(&route, wayfinder::Method::Get) {
            Ok(Match::Route(routes::Route::People(routes::people::Route::Index(_)))) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn test_people_delete() {
        let route = "/people/12345678901234567890123456789012";
        match routes::match_route(&route, wayfinder::Method::Delete) {
            Ok(Match::Route(routes::Route::People(routes::people::Route::Destroy(_)))) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn test_not_allowed() {
        let route = "/people/12345678901234567890123456789012";
        match routes::match_route(&route, wayfinder::Method::Post) {
            Ok(Match::NotAllowed) => {}
            _ => assert!(false),
        }
    }
}
