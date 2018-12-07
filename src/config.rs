use std::fmt;

/// An entire routing file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteConfig {
    pub headers: Vec<Header>,
    pub routes: Routes,
}

impl RouteConfig {
    pub fn stringify(&self) -> String {
        let headers = self.headers.iter()
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
}

/// A bit of inline code above the route table.  Usually for `use` items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub text: String,
}

impl Header {
    pub fn new<S: AsRef<str>>(text: S) -> Header {
        let text = text.as_ref().to_string();
        Header { text }
    }
}

/// A listing of resources & routes.
#[derive(Debug, Clone, PartialEq, Eq)]
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
        let params = self.query_parameters.iter().map(|param| {
            format!("{}[{}]\n", indent(level), param)
        }).collect::<Vec<_>>().concat();
        let resources = self.resources.iter()
            .map(|r| r.stringify(level))
            .collect::<Vec<_>>()
            .concat();
        let nested_routes = self.routes.iter()
            .map(|r| r.stringify(level))
            .collect::<Vec<_>>()
            .concat();

        format!(
            "{}{}{}",
            params,
            resources,
            nested_routes,
        )
    }
}

/// A resource available at a specific path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub method: Method,
    pub name: String,
    pub is_redirect: bool,
    pub query_parameters: Vec<Param>,
}

impl Resource {
    pub fn stringify(&self, level: usize) -> String {
        let params = self.query_parameters.iter().map(|param| {
            format!("\n{}[{}]", indent(level+1), param)
        }).collect::<Vec<_>>().concat();

        format!(
            "{}{}{} {}{}\n",
            indent(level),
            self.method,
            if self.is_redirect { " ->" } else { "" },
            self.name,
            params,
        )
    }
}

/// A block of routes nested under a path segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedRoutes {
    pub path_segment: PathSegment,
    pub routes: Routes,
}

impl NestedRoutes {
    pub fn stringify(&self, level: usize) -> String {
        format!(
            "{}{}\n{}",
            indent(level),
            match self.path_segment {
                PathSegment::Static(ref p) => format!("{}", p),
                PathSegment::Dynamic(ref p) => format!("{{{}}}", p),
            },
            self.routes.stringify(level+1),
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
