//#![feature(test)]

mod routes;
//mod routes2;
//mod benches;

#[cfg(test)]
mod tests {
    use super::routes::routes;

    #[test]
    fn test_people() {
        let path = "/people";

        match routes::match_route(&path, b"GET") {
            Ok(wayfinder::Match::Route(routes::Route::People(routes::people::Route::Index(
                routes::people::Index { lang: None },
            )))) => {}
            _ => assert!(false, "route match error"),
        }
    }

    #[test]
    fn test_people2() {
        let path = "/people/";

        match routes::match_route(&path, b"GET") {
            Ok(wayfinder::Match::Route(routes::Route::People(routes::people::Route::Index(
                routes::people::Index { lang: None },
            )))) => {}
            _ => assert!(false, "route match error"),
        }
    }
}
