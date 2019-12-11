pub mod routes {

//! Application route configuration.
//!
//! Of note is the function [`match_route`] as well as request structs
//! specific to each named resource.
//!
//! Route configuration:
//!
//!     /
//!       foobar
//!         GET Foo::Bar
//!       fomo
//!         GET Fomo::AsUsual
//!       foosh
//!         GET Foosh::Ball
//!       {a: b}
//!         GET Bar::Dyn
//!
//! [`match_route`]: fn.match_route.html

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub mod bar {
    /// Renders for `GET /{a}`.
    #[derive(Debug)]
    pub struct Dyn {
        pub a: b,
    }

    impl Dyn {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            let Dyn { ref a, } = self;
            format!("/{}", a)
        }
    }

    /// Parameters for requests to the bar controller.
    #[derive(Debug)]
    pub enum Route {
        Dyn(Dyn),
    }

    impl Route {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            match self {
                Route::Dyn(ref route) => route.to_path(),
            }
        }
    }
}

pub mod fomo {
    /// Renders for `GET /fomo`.
    #[derive(Debug)]
    pub struct AsUsual;

    impl AsUsual {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            format!("/fomo")
        }
    }

    /// Parameters for requests to the fomo controller.
    #[derive(Debug)]
    pub enum Route {
        AsUsual(AsUsual),
    }

    impl Route {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            match self {
                Route::AsUsual(ref route) => route.to_path(),
            }
        }
    }
}

pub mod foo {
    /// Renders for `GET /foobar`.
    #[derive(Debug)]
    pub struct Bar;

    impl Bar {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            format!("/foobar")
        }
    }

    /// Parameters for requests to the foo controller.
    #[derive(Debug)]
    pub enum Route {
        Bar(Bar),
    }

    impl Route {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            match self {
                Route::Bar(ref route) => route.to_path(),
            }
        }
    }
}

pub mod foosh {
    /// Renders for `GET /foosh`.
    #[derive(Debug)]
    pub struct Ball;

    impl Ball {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            format!("/foosh")
        }
    }

    /// Parameters for requests to the foosh controller.
    #[derive(Debug)]
    pub enum Route {
        Ball(Ball),
    }

    impl Route {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            match self {
                Route::Ball(ref route) => route.to_path(),
            }
        }
    }
}

/// An active route in the application -- match against this.
#[derive(Debug)]
pub enum Route {
    Bar(bar::Route),
    Fomo(fomo::Route),
    Foo(foo::Route),
    Foosh(foosh::Route),
}

impl Route {
    /// Make a path to this route with the given parameters.
    pub fn to_path(&self) -> String {
        match self {
            Route::Bar(ref route) => route.to_path(),
            Route::Fomo(ref route) => route.to_path(),
            Route::Foo(ref route) => route.to_path(),
            Route::Foosh(ref route) => route.to_path(),
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
pub fn match_route<P: AsRef<[u8]>>(
    path: P,
    method: wayfinder::Method,
) -> Result<wayfinder::Match<Route>, wayfinder::Error> {
    use wayfinder::{Error, Method, Match};
    let mut path = std::str::from_utf8(path.as_ref()).unwrap().chars().fuse().peekable();
    if path.peek() == Some(&'/') {
        path.next();
    }

    let mut text = String::new();

    match path.next() {
        Some('f') => {
            match path.next() {
                Some('o') => {},
                _ => return Ok(Match::NotFound),
            }
            match path.next() {
                Some('m') => {
                    match path.next() {
                        Some('o') => {},
                        _ => return Ok(Match::NotFound),
                    }
                    match path.next() {
                        None => match method {
                            Method::Get => return Ok(Match::Route(Route::Fomo(fomo::Route::AsUsual(fomo::AsUsual {
                            })))),
                            _ => return Ok(Match::NotAllowed),
                        },
                        Some('/') => {}
                        _ => return Ok(Match::NotFound),
                    }
                    match path.next() {
                        None => match method {
                            Method::Get => return Ok(Match::Route(Route::Fomo(fomo::Route::AsUsual(fomo::AsUsual {
                            })))),
                            _ => return Ok(Match::NotAllowed),
                        },
                        _ => return Ok(Match::NotFound),
                    }
                },
                Some('o') => {
                    match path.next() {
                        Some('b') => {
                            match path.next() {
                                Some('a') => {},
                                _ => return Ok(Match::NotFound),
                            }
                            match path.next() {
                                Some('r') => {},
                                _ => return Ok(Match::NotFound),
                            }
                            match path.next() {
                                None => match method {
                                    Method::Get => return Ok(Match::Route(Route::Foo(foo::Route::Bar(foo::Bar {
                                    })))),
                                    _ => return Ok(Match::NotAllowed),
                                },
                                Some('/') => {}
                                _ => return Ok(Match::NotFound),
                            }
                            match path.next() {
                                None => match method {
                                    Method::Get => return Ok(Match::Route(Route::Foo(foo::Route::Bar(foo::Bar {
                                    })))),
                                    _ => return Ok(Match::NotAllowed),
                                },
                                _ => return Ok(Match::NotFound),
                            }
                        },
                        Some('s') => {
                            match path.next() {
                                Some('h') => {},
                                _ => return Ok(Match::NotFound),
                            }
                            match path.next() {
                                None => match method {
                                    Method::Get => return Ok(Match::Route(Route::Foosh(foosh::Route::Ball(foosh::Ball {
                                    })))),
                                    _ => return Ok(Match::NotAllowed),
                                },
                                Some('/') => {}
                                _ => return Ok(Match::NotFound),
                            }
                            match path.next() {
                                None => match method {
                                    Method::Get => return Ok(Match::Route(Route::Foosh(foosh::Route::Ball(foosh::Ball {
                                    })))),
                                    _ => return Ok(Match::NotAllowed),
                                },
                                _ => return Ok(Match::NotFound),
                            }
                        },
                        _ => return Ok(Match::NotFound),
                    }
                },
                _ => return Ok(Match::NotFound),
            }
        },
        None => return Ok(Match::NotFound),
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

    let a = text.parse()
        .map_err(|e| Error::fail("a", e))?;

    match path.next() {
        None => match method {
            Method::Get => return Ok(Match::Route(Route::Bar(bar::Route::Dyn(bar::Dyn {
                a,
            })))),
            _ => return Ok(Match::NotAllowed),
        },
        _ => return Ok(Match::NotFound),
    }
}

} // mod routes
