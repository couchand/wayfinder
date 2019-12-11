mod routes;

#[cfg(test)]
mod tests {
    use super::routes::routes;

    #[test]
    fn test_people() {
        let path = "/people";

        match routes::match_route(&path, wayfinder::Method::Get) {
            Ok(wayfinder::Match::Route(routes::Route::People(routes::people::Route::Index(routes::people::Index { lang: None })))) => {},
            _ => assert!(false, "route match error"),
        }
    }
}
