pub mod routes {

//! Application route configuration.
//!
//! Of note is the function [`match_route`] as well as request structs
//! specific to each named resource.
//!
//! Route configuration:
//!
//!     /
//!       [lang: String]
//!       users
//!         GET -> People::Index
//!       people
//!         GET People::Index
//!         POST People::Create
//!         new
//!           GET People::New
//!         {id: Uuid}
//!           GET People::Show
//!           PUT People::Update
//!             [name: String]
//!           DELETE People::Destroy
//!           edit
//!             GET People::Edit
//!       books
//!         GET Books::Index
//!
//! [`match_route`]: fn.match_route.html

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use uuid::Uuid;

pub mod books {
    use uuid::Uuid;

    /// Renders for `GET /books`.
    #[derive(Debug)]
    pub struct Index {
        pub lang: Option<String>,
    }

    impl Index {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            let Index { ref lang, } = self;
            format!("/books")
        }
    }

    /// Parameters for requests to the books controller.
    #[derive(Debug)]
    pub enum Route {
        Index(Index),
    }

    impl Route {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            match self {
                Route::Index(ref route) => route.to_path(),
            }
        }
    }
}

pub mod people {
    use uuid::Uuid;

    /// Renders for `POST /people`.
    #[derive(Debug)]
    pub struct Create {
        pub lang: Option<String>,
    }

    impl Create {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            let Create { ref lang, } = self;
            format!("/people")
        }
    }

    /// Renders for `DELETE /people/{id}`.
    #[derive(Debug)]
    pub struct Destroy {
        pub id: Uuid,
        pub lang: Option<String>,
    }

    impl Destroy {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            let Destroy { ref id, ref lang, } = self;
            format!("/people/{}", id)
        }
    }

    /// Renders for `GET /people/{id}/edit`.
    #[derive(Debug)]
    pub struct Edit {
        pub id: Uuid,
        pub lang: Option<String>,
    }

    impl Edit {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            let Edit { ref id, ref lang, } = self;
            format!("/people/{}/edit", id)
        }
    }

    /// Renders for `GET /people`.
    #[derive(Debug)]
    pub struct Index {
        pub lang: Option<String>,
    }

    impl Index {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            let Index { ref lang, } = self;
            format!("/people")
        }
    }

    /// Renders for `GET /people/new`.
    #[derive(Debug)]
    pub struct New {
        pub lang: Option<String>,
    }

    impl New {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            let New { ref lang, } = self;
            format!("/people/new")
        }
    }

    /// Renders for `GET /people/{id}`.
    #[derive(Debug)]
    pub struct Show {
        pub id: Uuid,
        pub lang: Option<String>,
    }

    impl Show {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            let Show { ref id, ref lang, } = self;
            format!("/people/{}", id)
        }
    }

    /// Renders for `PUT /people/{id}`.
    #[derive(Debug)]
    pub struct Update {
        pub id: Uuid,
        pub lang: Option<String>,
        pub name: Option<String>,
    }

    impl Update {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            let Update { ref id, ref lang, ref name, } = self;
            format!("/people/{}", id)
        }
    }

    /// Parameters for requests to the people controller.
    #[derive(Debug)]
    pub enum Route {
        Create(Create),
        Destroy(Destroy),
        Edit(Edit),
        Index(Index),
        New(New),
        Show(Show),
        Update(Update),
    }

    impl Route {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            match self {
                Route::Create(ref route) => route.to_path(),
                Route::Destroy(ref route) => route.to_path(),
                Route::Edit(ref route) => route.to_path(),
                Route::Index(ref route) => route.to_path(),
                Route::New(ref route) => route.to_path(),
                Route::Show(ref route) => route.to_path(),
                Route::Update(ref route) => route.to_path(),
            }
        }
    }
}

/// An active route in the application -- match against this.
#[derive(Debug)]
pub enum Route {
    Books(books::Route),
    People(people::Route),
}

impl Route {
    /// Make a path to this route with the given parameters.
    pub fn to_path(&self) -> String {
        match self {
            Route::Books(ref route) => route.to_path(),
            Route::People(ref route) => route.to_path(),
        }
    }
}

/// Match a path and method against this router.
///
/// Accepts an iterator for the characters of the request path,
/// as well as a [`wayfinder::Method`] for the HTTP verb.
/// Returns a `Result`, usually `Ok` with the result of the
/// [`wayfinder::Match`].
///
/// If the match was successful, it will be a `Match::Route` or
/// `Match::Redirect` with the parameters enclosed.  You can then
/// match on the [`Route`] to pass control of the request along to
/// a specific handler.
///
/// If there is no match, this will return `Match::NotFound`
/// if no path matches (which you could return as `404 Not Found`),
/// or `Match::NotAllowed` if no method matches (in which case a
/// `405 Not Allowed` would be appropriate).
///
/// If a route parameter fails to parse correctly, this will return
/// `Err` with the underlying parsing error.  Usually you'll want
/// to send back a `400 Bad Request` for that.
///
/// [`wayfinder::Method`]: ../../wayfinder/enum.Method.html
/// [`wayfinder::Match`]: ../../wayfinder/enum.Match.html
/// [`Route`]: enum.Route.html
pub fn match_route<P: AsRef<str>>(
    path: P,
    method: wayfinder::Method,
) -> Result<wayfinder::Match<Route>, wayfinder::Error> {
    use wayfinder::{Error, Method, Match};
    let mut path = path.as_ref().chars().fuse().peekable();
    if path.peek() == Some(&'/') {
        path.next();
    }
    match path.next() {
        Some('b') => {
            match path.next() {
                Some('o') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('o') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('k') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('s') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                None => match method {
                    Method::Get => return Ok(Match::Route(Route::Books(books::Route::Index(books::Index {
                        lang: None,
                    })))),
                    _ => return Ok(Match::NotAllowed),
                },
                Some('/') => {}
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                None => match method {
                    Method::Get => return Ok(Match::Route(Route::Books(books::Route::Index(books::Index {
                        lang: None,
                    })))),
                    _ => return Ok(Match::NotAllowed),
                },
                _ => return Ok(Match::NotFound),
            }
        },
        Some('p') => {
            match path.next() {
                Some('e') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('o') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('p') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('l') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('e') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                None => match method {
                    Method::Get => return Ok(Match::Route(Route::People(people::Route::Index(people::Index {
                        lang: None,
                    })))),
                    Method::Post => return Ok(Match::Route(Route::People(people::Route::Create(people::Create {
                        lang: None,
                    })))),
                    _ => return Ok(Match::NotAllowed),
                },
                Some('/') => {}
                _ => return Ok(Match::NotFound),
            }

            let mut text = String::new();

            match path.next() {
                None => match method {
                    Method::Get => return Ok(Match::Route(Route::People(people::Route::Index(people::Index {
                        lang: None,
                    })))),
                    Method::Post => return Ok(Match::Route(Route::People(people::Route::Create(people::Create {
                        lang: None,
                    })))),
                    _ => return Ok(Match::NotAllowed),
                },
                Some('n') => {
                    match path.next() {
                        Some('e') => {},
                        _ => return Ok(Match::NotFound),
                    }
                    match path.next() {
                        Some('w') => {},
                        _ => return Ok(Match::NotFound),
                    }
                    match path.next() {
                        None => match method {
                            Method::Get => return Ok(Match::Route(Route::People(people::Route::New(people::New {
                                lang: None,
                            })))),
                            _ => return Ok(Match::NotAllowed),
                        },
                        Some('/') => {}
                        _ => return Ok(Match::NotFound),
                    }
                    match path.next() {
                        None => match method {
                            Method::Get => return Ok(Match::Route(Route::People(people::Route::New(people::New {
                                lang: None,
                            })))),
                            _ => return Ok(Match::NotAllowed),
                        },
                        _ => return Ok(Match::NotFound),
                    }
                },
                Some(c) => text.push(c),
            }

            loop {
                match path.peek().cloned() {
                    None => break,
                    Some(c) => {
                        path.next();
                        if c == '/' {
                            break;
                        } else {
                            text.push(c);
                        }
                    },
                }
            };

            let id = text.parse()
                .map_err(|e| Error::fail("id", e))?;

            match path.next() {
                None => match method {
                    Method::Get => return Ok(Match::Route(Route::People(people::Route::Show(people::Show {
                        id,
                        lang: None,
                    })))),
                    Method::Put => return Ok(Match::Route(Route::People(people::Route::Update(people::Update {
                        id,
                        lang: None,
                        name: None,
                    })))),
                    Method::Delete => return Ok(Match::Route(Route::People(people::Route::Destroy(people::Destroy {
                        id,
                        lang: None,
                    })))),
                    _ => return Ok(Match::NotAllowed),
                },
                Some('e') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('d') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('i') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('t') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                None => match method {
                    Method::Get => return Ok(Match::Route(Route::People(people::Route::Edit(people::Edit {
                        id,
                        lang: None,
                    })))),
                    _ => return Ok(Match::NotAllowed),
                },
                Some('/') => {}
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                None => match method {
                    Method::Get => return Ok(Match::Route(Route::People(people::Route::Edit(people::Edit {
                        id,
                        lang: None,
                    })))),
                    _ => return Ok(Match::NotAllowed),
                },
                _ => return Ok(Match::NotFound),
            }
        },
        Some('u') => {
            match path.next() {
                Some('s') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('e') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('r') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('s') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                None => match method {
                    Method::Get => return Ok(Match::Redirect(Route::People(people::Route::Index(people::Index {
                        lang: None,
                    })))),
                    _ => return Ok(Match::NotAllowed),
                },
                Some('/') => {}
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                None => match method {
                    Method::Get => return Ok(Match::Redirect(Route::People(people::Route::Index(people::Index {
                        lang: None,
                    })))),
                    _ => return Ok(Match::NotAllowed),
                },
                _ => return Ok(Match::NotFound),
            }
        },
        _ => return Ok(Match::NotFound),
    }
}

} // mod routes
