use std::fmt;

/// An entire routing file.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RouteConfig {
    pub headers: Vec<Header>,
    pub routes: Routes,
}

impl RouteConfig {
    pub fn stringify(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|h| format!("{}\n", h.text))
            .collect::<Vec<_>>()
            .concat();

        format!(
            "{}{}/\n{}",
            headers,
            if headers != "" { "\n" } else { "" },
            self.routes.stringify(1),
        )
    }

    pub fn mount<S: AsRef<str>>(mut self, at: S, config: RouteConfig) -> RouteConfig {
        use itertools::Itertools;

        let RouteConfig { headers, routes } = config;

        self.headers.extend(headers.into_iter());
        self.headers = self.headers.into_iter().unique().collect();
        self.routes.routes.push(NestedRoutes::new(at, routes));

        self
    }
}

/// A bit of inline code above the route table.  Usually for `use` items.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Header {
    pub text: String,
}

#[macro_export]
macro_rules! header {
    (
        $($tokens:item)*
    ) => {
        ::wayfinder_core::Header::new(stringify!($($tokens)*))
    }
}

impl Header {
    pub fn new<S: AsRef<str>>(text: S) -> Header {
        let text = text.as_ref().to_string();
        Header { text }
    }
}

/// A listing of resources & routes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Routes {
    pub resources: Vec<Resource>,
    pub routes: Vec<NestedRoutes>,
    pub query_parameters: Vec<Param>,
}

fn indent(level: usize) -> &'static str {
    match level {
        0 => "",
        1 => "  ",
        2 => "    ",
        3 => "      ",
        4 => "        ",
        5 => "          ",
        6 => "            ",
        7 => "              ",
        _ => "              --",
    }
}

impl Routes {
    pub fn stringify(&self, level: usize) -> String {
        let params = self
            .query_parameters
            .iter()
            .map(|param| format!("{}[{}]\n", indent(level), param))
            .collect::<Vec<_>>()
            .concat();
        let resources = self
            .resources
            .iter()
            .map(|r| r.stringify(level))
            .collect::<Vec<_>>()
            .concat();
        let nested_routes = self
            .routes
            .iter()
            .map(|r| r.stringify(level))
            .collect::<Vec<_>>()
            .concat();

        format!("{}{}{}", params, resources, nested_routes,)
    }
}

/// A resource available at a specific path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub method: Method,
    pub controller: String,
    pub action: String,
    pub is_redirect: bool,
    pub query_parameters: Vec<Param>,
}

impl Resource {
    pub fn stringify(&self, level: usize) -> String {
        let params = self
            .query_parameters
            .iter()
            .map(|param| format!("\n{}[{}]", indent(level + 1), param))
            .collect::<Vec<_>>()
            .concat();

        format!(
            "{}{}{} {}::{}{}\n",
            indent(level),
            self.method,
            if self.is_redirect { " ->" } else { "" },
            self.controller,
            self.action,
            params,
        )
    }
}

#[macro_export]
macro_rules! get {
    (
        $controller:ident :: $action:ident
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Get,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: false,
            query_parameters: vec![],
        }
    };
    (
        -> $controller:ident :: $action:ident
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Get,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: true,
            query_parameters: vec![],
        }
    }
}

#[macro_export]
macro_rules! post {
    (
        $controller:ident :: $action:ident
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Post,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: false,
            query_parameters: vec![],
        }
    }
}

#[macro_export]
macro_rules! put {
    (
        $controller:ident :: $action:ident
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Put,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: false,
            query_parameters: vec![],
        }
    };
    (
        $controller:ident :: $action:ident , $($params:expr),+
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Put,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: false,
            query_parameters: vec![$($params),+],
        }
    }
}

#[macro_export]
macro_rules! delete {
    (
        $controller:ident :: $action:ident
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Delete,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: false,
            query_parameters: vec![],
        }
    }
}

/// A block of routes nested under a path segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedRoutes {
    pub path_segment: PathSegment,
    pub routes: Routes,
}

impl NestedRoutes {
    pub fn new<P: Into<PathSegment>>(path_segment: P, routes: Routes) -> NestedRoutes {
        NestedRoutes {
            path_segment: path_segment.into(),
            routes,
        }
    }

    pub fn stringify(&self, level: usize) -> String {
        format!(
            "{}{}\n{}",
            indent(level),
            match self.path_segment {
                PathSegment::Static(ref p) => format!("{}", p),
                PathSegment::Dynamic(ref p) => format!("{{{}}}", p),
            },
            self.routes.stringify(level + 1),
        )
    }
}

/// A path segment is either a static string or a dynamic parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSegment {
    Static(String),
    Dynamic(Param),
}

impl<T: AsRef<str>> From<T> for PathSegment {
    fn from(s: T) -> PathSegment {
        PathSegment::Static(s.as_ref().to_string())
    }
}

impl From<Param> for PathSegment {
    fn from(param: Param) -> PathSegment {
        PathSegment::Dynamic(param)
    }
}

impl fmt::Display for PathSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PathSegment::Static(s) => f.write_str(s),
            PathSegment::Dynamic(p) => p.fmt(f),
        }
    }
}

/// Path and query parameters have a name and type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub typ: String, // TODO: something else?
}

impl Param {
    pub fn new<S: AsRef<str>, T: AsRef<str>>(name: S, typ: T) -> Param {
        Param {
            name: name.as_ref().to_string(),
            typ: typ.as_ref().to_string(),
        }
    }
}
impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name)?;
        f.write_str(": ")?;
        f.write_str(&self.typ)
    }
}

#[macro_export]
macro_rules! param {
    (
        $name:ident : $type:ty
    ) => {
        ::wayfinder_core::Param::new(stringify!($name), stringify!($type))
    }
}

/// HTTP methods that resources can respond to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    // TODO: more?
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
        })
    }
}

#[cfg(feature = "http")]
impl std::convert::From<&http::Method> for Method {
    fn from(method: &http::Method) -> Method {
        match method {
            &http::Method::GET => Method::Get,
            &http::Method::POST => Method::Post,
            &http::Method::PUT => Method::Put,
            &http::Method::DELETE => Method::Delete,
            _ => unimplemented!("Method unimplemented: {}", method.as_str()),
        }
    }
}
