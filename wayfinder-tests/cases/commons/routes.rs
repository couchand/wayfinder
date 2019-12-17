pub mod routes {

    //! Application route configuration.
    //!
    //! Of note is the function [`match_route`] as well as request structs
    //! specific to each named resource.
    //!
    //! Route configuration:
    //!
    //! ```ignore
    //! /
    //!   foobar
    //!     GET Foo::Bar
    //!   fomo
    //!     GET Fomo::AsUsual
    //!   foosh
    //!     GET Foosh::Ball
    //!   {a: String}
    //!     GET Bar::Dyn
    //! ```
    //!
    //! [`match_route`]: fn.match_route.html

    #![allow(dead_code)]
    #![allow(unused_imports)]
    #![allow(unused_mut)]
    #![allow(unused_variables)]

    pub mod bar {
        /// Renders for `GET /{a}`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Dyn {
            pub a: String,
        }

        impl Dyn {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Dyn { ref a, } = self;
                format!("/{}", a)
            }
        }

        /// Parameters for requests to the bar controller.
        #[derive(Debug, PartialEq, Eq)]
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
        #[derive(Debug, PartialEq, Eq)]
        pub struct AsUsual;

        impl AsUsual {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                format!("/fomo")
            }
        }

        /// Parameters for requests to the fomo controller.
        #[derive(Debug, PartialEq, Eq)]
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
        #[derive(Debug, PartialEq, Eq)]
        pub struct Bar;

        impl Bar {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                format!("/foobar")
            }
        }

        /// Parameters for requests to the foo controller.
        #[derive(Debug, PartialEq, Eq)]
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
        #[derive(Debug, PartialEq, Eq)]
        pub struct Ball;

        impl Ball {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                format!("/foosh")
            }
        }

        /// Parameters for requests to the foosh controller.
        #[derive(Debug, PartialEq, Eq)]
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
    #[derive(Debug, PartialEq, Eq)]
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

    #[derive(PartialEq, Eq)]
    pub enum Match<T> {
        NotFound,
        NotAllowed,
        Route(T),
        Redirect(T),
    }

    use std::fmt;
    impl<T: fmt::Debug> fmt::Debug for Match<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Match::NotFound => write!(f, "Match::NotFound"),
                Match::NotAllowed => write!(f, "Match::NotAllowed"),
                Match::Route(t) => write!(f, "Match::Route({:?})", t),
                Match::Redirect(t) => write!(f, "Match::Redirect({:?})", t),
            }
        }
    }

    pub struct Error {
        param: String,
        what: Box<dyn fmt::Debug>,
    }

    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("wayfinder::Error")
                .field("param", &self.param)
                .field("what", &self.what)
                .finish()
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Error parsing '{}' parameter {:?}",
                self.param, self.what
            )
        }
    }

    impl std::error::Error for Error {}

    impl Error {
        pub fn fail<S: AsRef<str>, T: fmt::Debug + 'static>(param: S, what: T) -> Error {
            Error {
                param: param.as_ref().to_string(),
                what: Box::new(what),
            }
        }
    }
    /// Match a path and method against this router.
    ///
    /// Accepts a byte slice for the request path and HTTP verb.
    /// Returns a `Result`, usually `Ok` with the result of the
    /// [`Match`].
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
    /// [`Match`]: enum.Match.html
    /// [`Route`]: enum.Route.html
    pub fn match_route<P: AsRef<[u8]>, M: AsRef<[u8]>>(
        path: P,
        method: M,
    ) -> Result<Match<Route>, Error> {
        let method = method.as_ref();
        let path = path.as_ref();
        let len = path.len();
        let mut i = if len > 0 && &path[0..1] == b"/" { 1 } else { 0 };


        let start = i;

        if i + 3 <= len {
            match &path[i..i+3] {
                b"fo" => {
                    i += 3;
                    if i == len {
                        return Ok(Match::NotFound);
                    }
                    match &path[i..i+1] {
                        b"m" => {
                            i += 1;
                            if i == len {
                                return Ok(Match::NotFound);
                            }
                            match &path[i..i+1] {
                                b"o" => {
                                    i += 1;
                                },
                                _ => return Ok(Match::NotFound),
                            }
                            if i == len {
                                match method {
                                    b"GET" => return Ok(Match::Route(Route::Fomo(fomo::Route::AsUsual(fomo::AsUsual {
                                    })))),
                                    _ => return Ok(Match::NotAllowed),
                                }
                            }
                            match &path[i..i+1] {
                                b"/" => {
                                    i += 1;
                                },
                                _ => return Ok(Match::NotFound),
                            }
                            if i == len {
                                match method {
                                    b"GET" => return Ok(Match::Route(Route::Fomo(fomo::Route::AsUsual(fomo::AsUsual {
                                    })))),
                                    _ => return Ok(Match::NotAllowed),
                                }
                            }
                            return Ok(Match::NotFound);
                        },
                        b"o" => {
                            i += 1;
                            if i == len {
                                return Ok(Match::NotFound);
                            }
                            match &path[i..i+1] {
                                b"b" => {
                                    i += 1;
                                    if i + 2 > len {
                                        return Ok(Match::NotFound);
                                    }
                                    match &path[i..i+2] {
                                        b"ar" => {
                                            i += 2;
                                        },
                                        _ => return Ok(Match::NotFound),
                                    }
                                    if i == len {
                                        match method {
                                            b"GET" => return Ok(Match::Route(Route::Foo(foo::Route::Bar(foo::Bar {
                                            })))),
                                            _ => return Ok(Match::NotAllowed),
                                        }
                                    }
                                    match &path[i..i+1] {
                                        b"/" => {
                                            i += 1;
                                        },
                                        _ => return Ok(Match::NotFound),
                                    }
                                    if i == len {
                                        match method {
                                            b"GET" => return Ok(Match::Route(Route::Foo(foo::Route::Bar(foo::Bar {
                                            })))),
                                            _ => return Ok(Match::NotAllowed),
                                        }
                                    }
                                    return Ok(Match::NotFound);
                                },
                                b"s" => {
                                    i += 1;
                                    if i == len {
                                        return Ok(Match::NotFound);
                                    }
                                    match &path[i..i+1] {
                                        b"h" => {
                                            i += 1;
                                        },
                                        _ => return Ok(Match::NotFound),
                                    }
                                    if i == len {
                                        match method {
                                            b"GET" => return Ok(Match::Route(Route::Foosh(foosh::Route::Ball(foosh::Ball {
                                            })))),
                                            _ => return Ok(Match::NotAllowed),
                                        }
                                    }
                                    match &path[i..i+1] {
                                        b"/" => {
                                            i += 1;
                                        },
                                        _ => return Ok(Match::NotFound),
                                    }
                                    if i == len {
                                        match method {
                                            b"GET" => return Ok(Match::Route(Route::Foosh(foosh::Route::Ball(foosh::Ball {
                                            })))),
                                            _ => return Ok(Match::NotAllowed),
                                        }
                                    }
                                    return Ok(Match::NotFound);
                                },
                                _ => return Ok(Match::NotFound),
                            }
                        },
                        _ => return Ok(Match::NotFound),
                    }
                },
                _ => {},
            }
        }

        while i < len && &path[i..i+1] != b"/" {
            i += 1;
        }

        let text = std::str::from_utf8(&path[start..i]).unwrap();
        let a = text.parse()
            .map_err(|e| Error::fail("a", e))?;

        if i == len {
            match method {
                b"GET" => return Ok(Match::Route(Route::Bar(bar::Route::Dyn(bar::Dyn {
                    a,
                })))),
                _ => return Ok(Match::NotAllowed),
            }
        }
        match &path[i..i+1] {
            b"/" => {
                i += 1;
            },
            _ => return Ok(Match::NotFound),
        }
        if i == len {
            match method {
                b"GET" => return Ok(Match::Route(Route::Bar(bar::Route::Dyn(bar::Dyn {
                    a,
                })))),
                _ => return Ok(Match::NotAllowed),
            }
        }
        return Ok(Match::NotFound);
    }

} // mod routes
