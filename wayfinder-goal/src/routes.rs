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
    //!   [lang: String]
    //!   users
    //!     GET -> People::Index
    //!   people
    //!     GET People::Index
    //!     POST People::Create
    //!     new
    //!       GET People::New
    //!     {id: Uuid}
    //!       GET People::Show
    //!       PUT People::Update
    //!         [name: String]
    //!       DELETE People::Destroy
    //!       edit
    //!         GET People::Edit
    //!   books
    //!     GET Books::Index
    //!     POST Books::Create
    //!     new
    //!       GET Books::New
    //!     {id: Uuid}
    //!       GET Books::Show
    //!       PUT Books::update
    //!       DELETE Books::Destroy
    //!       edit
    //!         GET Books::Edit
    //! ```
    //!
    //! [`match_route`]: fn.match_route.html

    #![allow(dead_code)]
    #![allow(unused_imports)]
    #![allow(unused_mut)]
    #![allow(unused_variables)]

    use uuid::Uuid;

    pub mod books {
        use uuid::Uuid;

        /// Renders for `POST /books`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Create {
            pub lang: Option<String>,
        }

        impl Create {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Create { ref lang } = self;
                format!("/books")
            }
        }

        /// Renders for `DELETE /books/{id}`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Destroy {
            pub id: Uuid,
            pub lang: Option<String>,
        }

        impl Destroy {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Destroy { ref id, ref lang } = self;
                format!("/books/{}", id)
            }
        }

        /// Renders for `GET /books/{id}/edit`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Edit {
            pub id: Uuid,
            pub lang: Option<String>,
        }

        impl Edit {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Edit { ref id, ref lang } = self;
                format!("/books/{}/edit", id)
            }
        }

        /// Renders for `GET /books`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Index {
            pub lang: Option<String>,
        }

        impl Index {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Index { ref lang } = self;
                format!("/books")
            }
        }

        /// Renders for `GET /books/new`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct New {
            pub lang: Option<String>,
        }

        impl New {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let New { ref lang } = self;
                format!("/books/new")
            }
        }

        /// Renders for `GET /books/{id}`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Show {
            pub id: Uuid,
            pub lang: Option<String>,
        }

        impl Show {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Show { ref id, ref lang } = self;
                format!("/books/{}", id)
            }
        }

        /// Renders for `PUT /books/{id}`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Update {
            pub id: Uuid,
            pub lang: Option<String>,
        }

        impl Update {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Update { ref id, ref lang } = self;
                format!("/books/{}", id)
            }
        }

        /// Parameters for requests to the books controller.
        #[derive(Debug, PartialEq, Eq)]
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

    pub mod people {
        use uuid::Uuid;

        /// Renders for `POST /people`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Create {
            pub lang: Option<String>,
        }

        impl Create {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Create { ref lang } = self;
                format!("/people")
            }
        }

        /// Renders for `DELETE /people/{id}`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Destroy {
            pub id: Uuid,
            pub lang: Option<String>,
        }

        impl Destroy {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Destroy { ref id, ref lang } = self;
                format!("/people/{}", id)
            }
        }

        /// Renders for `GET /people/{id}/edit`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Edit {
            pub id: Uuid,
            pub lang: Option<String>,
        }

        impl Edit {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Edit { ref id, ref lang } = self;
                format!("/people/{}/edit", id)
            }
        }

        /// Renders for `GET /people`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Index {
            pub lang: Option<String>,
        }

        impl Index {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Index { ref lang } = self;
                format!("/people")
            }
        }

        /// Renders for `GET /people/new`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct New {
            pub lang: Option<String>,
        }

        impl New {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let New { ref lang } = self;
                format!("/people/new")
            }
        }

        /// Renders for `GET /people/{id}`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Show {
            pub id: Uuid,
            pub lang: Option<String>,
        }

        impl Show {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Show { ref id, ref lang } = self;
                format!("/people/{}", id)
            }
        }

        /// Renders for `PUT /people/{id}`.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Update {
            pub id: Uuid,
            pub lang: Option<String>,
            pub name: Option<String>,
        }

        impl Update {
            /// Make a path to this route with the given parameters.
            pub fn to_path(&self) -> String {
                let Update {
                    ref id,
                    ref lang,
                    ref name,
                } = self;
                format!("/people/{}", id)
            }
        }

        /// Parameters for requests to the people controller.
        #[derive(Debug, PartialEq, Eq)]
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
    #[derive(Debug, PartialEq, Eq)]
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

        if i == len {
            return Ok(Match::NotFound);
        }
        match &path[i..i + 1] {
            b"b" => {
                i += 1;
                if i + 4 > len {
                    return Ok(Match::NotFound);
                }
                match &path[i..i + 4] {
                    b"ooks" => {
                        i += 4;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::Books(books::Route::Index(
                                books::Index { lang: None },
                            ))))
                        }
                        b"POST" => {
                            return Ok(Match::Route(Route::Books(books::Route::Create(
                                books::Create { lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                match &path[i..i + 1] {
                    b"/" => {
                        i += 1;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::Books(books::Route::Index(
                                books::Index { lang: None },
                            ))))
                        }
                        b"POST" => {
                            return Ok(Match::Route(Route::Books(books::Route::Create(
                                books::Create { lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }

                let start = i;

                if i + 3 <= len {
                    match &path[i..i + 3] {
                        b"new" => {
                            i += 3;
                            if i == len {
                                match method {
                                    b"GET" => {
                                        return Ok(Match::Route(Route::Books(books::Route::New(
                                            books::New { lang: None },
                                        ))))
                                    }
                                    _ => return Ok(Match::NotAllowed),
                                }
                            }
                            match &path[i..i + 1] {
                                b"/" => {
                                    i += 1;
                                }
                                _ => return Ok(Match::NotFound),
                            }
                            if i == len {
                                match method {
                                    b"GET" => {
                                        return Ok(Match::Route(Route::Books(books::Route::New(
                                            books::New { lang: None },
                                        ))))
                                    }
                                    _ => return Ok(Match::NotAllowed),
                                }
                            }
                            return Ok(Match::NotFound);
                        }
                        _ => {}
                    }
                }

                while i < len && &path[i..i + 1] != b"/" {
                    i += 1;
                }

                let text = std::str::from_utf8(&path[start..i]).unwrap();
                let id = text.parse().map_err(|e| Error::fail("id", e))?;

                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::Books(books::Route::Show(
                                books::Show { id, lang: None },
                            ))))
                        }
                        b"PUT" => {
                            return Ok(Match::Route(Route::Books(books::Route::Update(
                                books::Update { id, lang: None },
                            ))))
                        }
                        b"DELETE" => {
                            return Ok(Match::Route(Route::Books(books::Route::Destroy(
                                books::Destroy { id, lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                match &path[i..i + 1] {
                    b"/" => {
                        i += 1;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::Books(books::Route::Show(
                                books::Show { id, lang: None },
                            ))))
                        }
                        b"PUT" => {
                            return Ok(Match::Route(Route::Books(books::Route::Update(
                                books::Update { id, lang: None },
                            ))))
                        }
                        b"DELETE" => {
                            return Ok(Match::Route(Route::Books(books::Route::Destroy(
                                books::Destroy { id, lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                if i + 4 > len {
                    return Ok(Match::NotFound);
                }
                match &path[i..i + 4] {
                    b"edit" => {
                        i += 4;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::Books(books::Route::Edit(
                                books::Edit { id, lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                match &path[i..i + 1] {
                    b"/" => {
                        i += 1;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::Books(books::Route::Edit(
                                books::Edit { id, lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                return Ok(Match::NotFound);
            }
            b"p" => {
                i += 1;
                if i + 5 > len {
                    return Ok(Match::NotFound);
                }
                match &path[i..i + 5] {
                    b"eople" => {
                        i += 5;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::People(people::Route::Index(
                                people::Index { lang: None },
                            ))))
                        }
                        b"POST" => {
                            return Ok(Match::Route(Route::People(people::Route::Create(
                                people::Create { lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                match &path[i..i + 1] {
                    b"/" => {
                        i += 1;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::People(people::Route::Index(
                                people::Index { lang: None },
                            ))))
                        }
                        b"POST" => {
                            return Ok(Match::Route(Route::People(people::Route::Create(
                                people::Create { lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }

                let start = i;

                if i + 3 <= len {
                    match &path[i..i + 3] {
                        b"new" => {
                            i += 3;
                            if i == len {
                                match method {
                                    b"GET" => {
                                        return Ok(Match::Route(Route::People(people::Route::New(
                                            people::New { lang: None },
                                        ))))
                                    }
                                    _ => return Ok(Match::NotAllowed),
                                }
                            }
                            match &path[i..i + 1] {
                                b"/" => {
                                    i += 1;
                                }
                                _ => return Ok(Match::NotFound),
                            }
                            if i == len {
                                match method {
                                    b"GET" => {
                                        return Ok(Match::Route(Route::People(people::Route::New(
                                            people::New { lang: None },
                                        ))))
                                    }
                                    _ => return Ok(Match::NotAllowed),
                                }
                            }
                            return Ok(Match::NotFound);
                        }
                        _ => {}
                    }
                }

                while i < len && &path[i..i + 1] != b"/" {
                    i += 1;
                }

                let text = std::str::from_utf8(&path[start..i]).unwrap();
                let id = text.parse().map_err(|e| Error::fail("id", e))?;

                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::People(people::Route::Show(
                                people::Show { id, lang: None },
                            ))))
                        }
                        b"PUT" => {
                            return Ok(Match::Route(Route::People(people::Route::Update(
                                people::Update {
                                    id,
                                    lang: None,
                                    name: None,
                                },
                            ))))
                        }
                        b"DELETE" => {
                            return Ok(Match::Route(Route::People(people::Route::Destroy(
                                people::Destroy { id, lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                match &path[i..i + 1] {
                    b"/" => {
                        i += 1;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::People(people::Route::Show(
                                people::Show { id, lang: None },
                            ))))
                        }
                        b"PUT" => {
                            return Ok(Match::Route(Route::People(people::Route::Update(
                                people::Update {
                                    id,
                                    lang: None,
                                    name: None,
                                },
                            ))))
                        }
                        b"DELETE" => {
                            return Ok(Match::Route(Route::People(people::Route::Destroy(
                                people::Destroy { id, lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                if i + 4 > len {
                    return Ok(Match::NotFound);
                }
                match &path[i..i + 4] {
                    b"edit" => {
                        i += 4;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::People(people::Route::Edit(
                                people::Edit { id, lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                match &path[i..i + 1] {
                    b"/" => {
                        i += 1;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Route(Route::People(people::Route::Edit(
                                people::Edit { id, lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                return Ok(Match::NotFound);
            }
            b"u" => {
                i += 1;
                if i + 4 > len {
                    return Ok(Match::NotFound);
                }
                match &path[i..i + 4] {
                    b"sers" => {
                        i += 4;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Redirect(Route::People(people::Route::Index(
                                people::Index { lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                match &path[i..i + 1] {
                    b"/" => {
                        i += 1;
                    }
                    _ => return Ok(Match::NotFound),
                }
                if i == len {
                    match method {
                        b"GET" => {
                            return Ok(Match::Redirect(Route::People(people::Route::Index(
                                people::Index { lang: None },
                            ))))
                        }
                        _ => return Ok(Match::NotAllowed),
                    }
                }
                return Ok(Match::NotFound);
            }
            _ => return Ok(Match::NotFound),
        }
    }
} // mod routes
