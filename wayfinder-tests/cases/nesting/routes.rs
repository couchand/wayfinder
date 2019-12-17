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
    //!   GET Index
    //!   {id: Uuid}
    //!     GET Admin::People::Show
    //! ```
    //!
    //! [`match_route`]: fn.match_route.html

    #![allow(dead_code)]
    #![allow(unused_imports)]
    #![allow(unused_mut)]
    #![allow(unused_variables)]

    use uuid::Uuid;

    /// Renders for `GET /`.
    #[derive(Debug, PartialEq, Eq)]
    pub struct Index;

    impl Index {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            format!("/")
        }
    }

    pub mod admin {
        use uuid::Uuid;

        pub mod people {
            use uuid::Uuid;

            /// Renders for `GET /{id}`.
            #[derive(Debug, PartialEq, Eq)]
            pub struct Show {
                pub id: Uuid,
            }

            impl Show {
                /// Make a path to this route with the given parameters.
                pub fn to_path(&self) -> String {
                    let Show { ref id, } = self;
                    format!("/{}", id)
                }
            }

            /// Parameters for requests to the people controller.
            #[derive(Debug, PartialEq, Eq)]
            pub enum Route {
                Show(Show),
            }

            impl Route {
                /// Make a path to this route with the given parameters.
                pub fn to_path(&self) -> String {
                    match self {
                        Route::Show(ref route) => route.to_path(),
                    }
                }
            }
        }

        /// Parameters for requests to the admin controller.
        #[derive(Debug, PartialEq, Eq)]
        pub enum Route {
            People(people::Route),
        }

        impl Route {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                match self {
                    Route::People(ref route) => route.to_path(),
                }
            }
        }
    }

    /// Parameters for requests to the routes controller.
    #[derive(Debug, PartialEq, Eq)]
    pub enum Route {
        Index(Index),
        Admin(admin::Route),
    }

    impl Route {
        /// Make a path to this route with the given parameters.
        pub fn to_path(&self) -> String {
            match self {
                Route::Index(ref route) => route.to_path(),
                Route::Admin(ref route) => route.to_path(),
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
    pub fn match_route<P: AsRef<[u8]>, M: AsRef<[u8]>>(
        path: P,
        method: M,
    ) -> Result<wayfinder::Match<Route>, wayfinder::Error> {
        use wayfinder::{Error, Match};

        let method = method.as_ref();
        let path = path.as_ref();
        let len = path.len();
        let mut i = if len > 0 && &path[0..1] == b"/" { 1 } else { 0 };

        if i == len {
            match method {
                b"GET" => return Ok(Match::Route(Route::Index(Index {
                }))),
                _ => return Ok(Match::NotAllowed),
            }
        }
        let start = i;

        while i < len && &path[i..i+1] != b"/" {
            i += 1;
        }

        let text = std::str::from_utf8(&path[start..i]).unwrap();
        let id = text.parse()
            .map_err(|e| Error::fail("id", e))?;

        if i == len {
            match method {
                b"GET" => return Ok(Match::Route(Route::Admin(admin::Route::People(admin::people::Route::Show(admin::people::Show {
                    id,
                }))))),
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
                b"GET" => return Ok(Match::Route(Route::Admin(admin::Route::People(admin::people::Route::Show(admin::people::Show {
                    id,
                }))))),
                _ => return Ok(Match::NotAllowed),
            }
        }
        return Ok(Match::NotFound);
    }

} // mod routes
