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
    use uuid::Uuid;
    use wayfinder::Match;

    fn test_cases() -> Vec<(&'static str, wayfinder::Method, Match<routes::Route>)> {
        let id: Uuid = "12345678901234567890123456789012".parse().unwrap();
        vec![
            (
                "",
                wayfinder::Method::Get,
                Match::NotFound,
            ),
            (
                "/",
                wayfinder::Method::Get,
                Match::NotFound,
            ),
            (
                "/user",
                wayfinder::Method::Get,
                Match::NotFound,
            ),
            (
                "/users",
                wayfinder::Method::Get,
                Match::Redirect(routes::Route::People(routes::people::Route::Index(
                    routes::people::Index { lang: None },
                ))),
            ),
            (
                "/users/",
                wayfinder::Method::Get,
                Match::Redirect(routes::Route::People(routes::people::Route::Index(
                    routes::people::Index { lang: None },
                ))),
            ),
            (
                "/people",
                wayfinder::Method::Get,
                Match::Route(routes::Route::People(routes::people::Route::Index(
                    routes::people::Index { lang: None },
                ))),
            ),
            (
                "/people/",
                wayfinder::Method::Get,
                Match::Route(routes::Route::People(routes::people::Route::Index(
                    routes::people::Index { lang: None },
                ))),
            ),
            (
                "/people",
                wayfinder::Method::Post,
                Match::Route(routes::Route::People(routes::people::Route::Create(
                    routes::people::Create { lang: None },
                ))),
            ),
            (
                "/people/",
                wayfinder::Method::Post,
                Match::Route(routes::Route::People(routes::people::Route::Create(
                    routes::people::Create { lang: None },
                ))),
            ),
            (
                "/people/new",
                wayfinder::Method::Get,
                Match::Route(routes::Route::People(routes::people::Route::New(
                    routes::people::New { lang: None },
                ))),
            ),
            (
                "/people/new/",
                wayfinder::Method::Get,
                Match::Route(routes::Route::People(routes::people::Route::New(
                    routes::people::New { lang: None },
                ))),
            ),
            (
                "/people/12345678901234567890123456789012",
                wayfinder::Method::Get,
                Match::Route(routes::Route::People(routes::people::Route::Show(
                    routes::people::Show { id, lang: None },
                ))),
            ),
            (
                "/people/12345678901234567890123456789012/",
                wayfinder::Method::Get,
                Match::Route(routes::Route::People(routes::people::Route::Show(
                    routes::people::Show { id, lang: None },
                ))),
            ),
            (
                "/people/12345678901234567890123456789012",
                wayfinder::Method::Put,
                Match::Route(routes::Route::People(routes::people::Route::Update(
                    routes::people::Update {
                        id,
                        lang: None,
                        name: None,
                    },
                ))),
            ),
            (
                "/people/12345678901234567890123456789012/",
                wayfinder::Method::Put,
                Match::Route(routes::Route::People(routes::people::Route::Update(
                    routes::people::Update {
                        id,
                        lang: None,
                        name: None,
                    },
                ))),
            ),
            (
                "/people/12345678901234567890123456789012",
                wayfinder::Method::Delete,
                Match::Route(routes::Route::People(routes::people::Route::Destroy(
                    routes::people::Destroy { id, lang: None },
                ))),
            ),
            (
                "/people/12345678901234567890123456789012/",
                wayfinder::Method::Delete,
                Match::Route(routes::Route::People(routes::people::Route::Destroy(
                    routes::people::Destroy { id, lang: None },
                ))),
            ),
            (
                "/people/12345678901234567890123456789012/edit",
                wayfinder::Method::Get,
                Match::Route(routes::Route::People(routes::people::Route::Edit(
                    routes::people::Edit { id, lang: None },
                ))),
            ),
            (
                "/people/12345678901234567890123456789012/edit/",
                wayfinder::Method::Get,
                Match::Route(routes::Route::People(routes::people::Route::Edit(
                    routes::people::Edit { id, lang: None },
                ))),
            ),
            (
                "/books",
                wayfinder::Method::Get,
                Match::Route(routes::Route::Books(routes::books::Route::Index(
                    routes::books::Index { lang: None },
                ))),
            ),
            (
                "/books/",
                wayfinder::Method::Get,
                Match::Route(routes::Route::Books(routes::books::Route::Index(
                    routes::books::Index { lang: None },
                ))),
            ),
            (
                "/books",
                wayfinder::Method::Post,
                Match::Route(routes::Route::Books(routes::books::Route::Create(
                    routes::books::Create { lang: None },
                ))),
            ),
            (
                "/books/",
                wayfinder::Method::Post,
                Match::Route(routes::Route::Books(routes::books::Route::Create(
                    routes::books::Create { lang: None },
                ))),
            ),
            (
                "/books/new",
                wayfinder::Method::Get,
                Match::Route(routes::Route::Books(routes::books::Route::New(
                    routes::books::New { lang: None },
                ))),
            ),
            (
                "/books/new/",
                wayfinder::Method::Get,
                Match::Route(routes::Route::Books(routes::books::Route::New(
                    routes::books::New { lang: None },
                ))),
            ),
            (
                "/books/12345678901234567890123456789012",
                wayfinder::Method::Get,
                Match::Route(routes::Route::Books(routes::books::Route::Show(
                    routes::books::Show { id, lang: None },
                ))),
            ),
            (
                "/books/12345678901234567890123456789012/",
                wayfinder::Method::Get,
                Match::Route(routes::Route::Books(routes::books::Route::Show(
                    routes::books::Show { id, lang: None },
                ))),
            ),
            (
                "/books/12345678901234567890123456789012",
                wayfinder::Method::Put,
                Match::Route(routes::Route::Books(routes::books::Route::Update(
                    routes::books::Update { id, lang: None },
                ))),
            ),
            (
                "/books/12345678901234567890123456789012/",
                wayfinder::Method::Put,
                Match::Route(routes::Route::Books(routes::books::Route::Update(
                    routes::books::Update { id, lang: None },
                ))),
            ),
            (
                "/books/12345678901234567890123456789012",
                wayfinder::Method::Delete,
                Match::Route(routes::Route::Books(routes::books::Route::Destroy(
                    routes::books::Destroy { id, lang: None },
                ))),
            ),
            (
                "/books/12345678901234567890123456789012/",
                wayfinder::Method::Delete,
                Match::Route(routes::Route::Books(routes::books::Route::Destroy(
                    routes::books::Destroy { id, lang: None },
                ))),
            ),
            (
                "/books/12345678901234567890123456789012/edit",
                wayfinder::Method::Get,
                Match::Route(routes::Route::Books(routes::books::Route::Edit(
                    routes::books::Edit { id, lang: None },
                ))),
            ),
            (
                "/books/12345678901234567890123456789012/edit/",
                wayfinder::Method::Get,
                Match::Route(routes::Route::Books(routes::books::Route::Edit(
                    routes::books::Edit { id, lang: None },
                ))),
            ),
        ]
    }

    #[test]
    fn test_routes() {
        for (route, method, expected) in test_cases() {
            let actual = match routes::match_route(&route, method) {
                Ok(m) => m,
                Err(e) => return assert!(false, "Unexpected error: {}", e),
            };
            assert_eq!(actual, expected);
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
