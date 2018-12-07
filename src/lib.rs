#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;

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

pub mod parse {
    // TODO: this parser is functional, but there's a lot more that can
    // be done to improve robustness and error reporting.

    use nom::{IResult, line_ending, not_line_ending};
    use nom::types::CompleteStr;

    use super::*;

    #[macro_use]
    pub mod errors {
        // This mod cribbed from ructe.

        use nom::ErrorKind;
        use std::sync::Mutex;

        macro_rules! err_str(
            ($msg:expr) => {{
                use self::errors::def_error;
                use nom::ErrorKind;
                lazy_static! {
                    static ref ERR: ErrorKind = def_error($msg);
                }
                ERR.clone()
            }}
        );

        pub fn def_error(msg: &'static str) -> ErrorKind {
            let mut errors = ERRORS.lock().unwrap();
            let n = errors.len();
            errors.push(msg);
            ErrorKind::Custom(n as u32)
        }

        pub fn get_error(n: u32) -> Option<String> {
            match ERRORS.lock() {
                Ok(e) => e.get(n as usize).map(|s| s.to_string()),
                Err(_) => None,
            }
        }

        lazy_static! {
            static ref ERRORS: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());
        }

        use std::fmt::Debug;
        use std::io::Write;

        use nom::{Context, Err};
        use nom::types::CompleteStr;

        pub fn show_errors<E>(
            out: &mut Write,
            buf: &str,
            result: nom::IResult<CompleteStr, E>,
            prefix: &str,
        ) where
            E: Debug,
        {
            match result {
                Ok(_) => (),
                Err(Err::Error(Context::Code(_before, e))) => {
                    show_error(out, buf, 0, &format!("error {:?}", e), prefix);
                }
                Err(Err::Error(Context::List(mut v))) => {
                    v.reverse();
                    for (i, e) in v {
                        let pos = buf.len() - i.len();
                        show_error(out, buf, pos, &get_message(&e), prefix);
                    }
                }
                Err(Err::Failure(Context::List(mut v))) => {
                    v.reverse();
                    for (i, e) in v {
                        let pos = buf.len() - i.len();
                        show_error(out, buf, pos, &get_message(&e), prefix);
                    }
                }
                Err(Err::Failure(e)) => {
                    show_error(out, buf, 0, &format!("failure {:?}", e), prefix);
                }
                Err(_) => show_error(out, buf, 0, "xyzzy", prefix),
            }
        }

        fn get_message(err: &ErrorKind) -> String {
            match err {
                &ErrorKind::Custom(n) => match get_error(n) {
                    Some(msg) => msg,
                    None => format!("Unknown error #{}", n),
                },
                err => format!("{:?}", err),
            }
        }

        fn show_error(
            out: &mut Write,
            buf: &str,
            pos: usize,
            msg: &str,
            prefix: &str,
        ) {
            let mut line_start = buf[0..pos].rsplitn(2, '\n');
            let _ = line_start.next();
            let line_start =
                line_start.next().map(|bytes| bytes.len() + 1).unwrap_or(0);
            let line = buf[line_start..]
                .splitn(2, '\n')
                .next()
                .unwrap();
            let line_no = what_line(buf, line_start);
            let pos_in_line = buf[line_start..pos].chars().count() + 1;
            writeln!(
                out,
                "{prefix}{:>4}:{}\n\
                 {prefix}     {:>pos$} {}",
                line_no,
                line,
                "^",
                msg,
                pos = pos_in_line,
                prefix = prefix,
            )
            .unwrap();
        }

        fn what_line(buf: &str, pos: usize) -> usize {
            1 + buf[0..pos].chars().filter(|c| *c == '\n').count()
        }
    }

    macro_rules! indented {
        (
            $name:ident < $ty:ident > , $($content:tt)*
        ) => {
            #[inline(always)]
            pub fn $name(input: CompleteStr, level: usize) -> IResult<CompleteStr, $ty> {
                    preceded!(input,
                        count!(tag!("  "), level),
                        $($content)*
                    )
            }
        }
    }

    pub fn is_identifier_char(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    named!(pub method<CompleteStr, Method>,
        alt_complete!(
            value!(Method::Get, tag_no_case!("get")) |
            value!(Method::Post, tag_no_case!("post")) |
            value!(Method::Put, tag_no_case!("put")) |
            value!(Method::Delete, tag_no_case!("delete"))
        )
    );
    named!(colon<CompleteStr, CompleteStr>,
        return_error!(
            err_str!("Expected \":\""),
            tag!(":")
        )
    );

    named!(pub param<CompleteStr, Param>,
        do_parse!(
            name: take_while1!(is_identifier_char) >>
            ws!(colon) >>
            typ: take_while1!(is_identifier_char) >>
            (Param::new(name, typ))
        )
    );

    indented!(path_segment<PathSegment>,
        alt_complete!(
            do_parse!(
                param: delimited!(
                    char!('{'),
                    ws!(param),
                    return_error!(
                        err_str!("Expected a \"}\""),
                        char!('}')
                    )
                ) >>
                (PathSegment::from(param))
            ) |
            do_parse!(
                text: take_while1!(is_identifier_char) >>
                (PathSegment::from(text))
            )
        )
    );

    indented!(query_parameter<Param>,
        delimited!(
            char!('['),
            ws!(param),
            return_error!(
                err_str!("Expected a \"]\""),
                char!(']')
            )
        )
    );

    named!(target<CompleteStr, (bool, String)>,
        alt_complete!(
            do_parse!(
                tag!("->") >>
                char!(' ') >>
                name: take_while1!(is_identifier_char) >>
                ((true, name.to_string()))
            ) |
            do_parse!(
                name: take_while1!(is_identifier_char) >>
                ((false, name.to_string()))
            )
        )
    );

    named!(require_newline<CompleteStr, ()>,
        return_error!(
            err_str!("Expected a newline"),
            value!((), many1!(line_ending))
        )
    );

    pub fn resource(
        input: CompleteStr,
        level: usize,
    ) -> IResult<CompleteStr, Resource> {
        preceded!(input,
            count!(tag!("  "), level),
            do_parse!(
                method: method >>
                char!(' ') >>
                target: target >>
                require_newline >>
                query_parameters: many0!(
                    terminated!(
                        apply!(query_parameter, level+1),
                        require_newline
                    )
                ) >>
                (Resource {
                    method,
                    name: target.1,
                    is_redirect: target.0,
                    query_parameters,
                })
            )
        )
    }

    pub fn resources_and_routes(
        input: CompleteStr,
        level: usize,
    ) -> IResult<CompleteStr, (Vec<Resource>, Vec<NestedRoutes>)> {
        return_error!(input,
            err_str!("Expected resources or child routes"),
            alt_complete!(
                do_parse!(
                    resources: many1!(
                        apply!(resource, level)
                    ) >>
                    routes: alt_complete!(
                        preceded!(
                            many0!(line_ending),
                            many1!(
                                apply!(nested_routes, level)
                            )
                        ) |
                        value!(vec![])
                    ) >>
                    ((resources, routes))
                ) |
                do_parse!(
                    routes: many1!(
                        apply!(nested_routes, level)
                    ) >>
                    ((vec![], routes))
                )
            )
        )
    }

    pub fn routes(
        input: CompleteStr,
        level: usize,
    ) -> IResult<CompleteStr, Routes> {
        do_parse!(input,
            query_parameters: many0!(
                terminated!(
                    apply!(query_parameter, level),
                    many1!(line_ending)
                )
            ) >>
            rnr: apply!(resources_and_routes, level) >>
            (Routes {
                resources: rnr.0,
                routes: rnr.1,
                query_parameters,
            })
        )
    }

    pub fn nested_routes(
        input: CompleteStr,
        level: usize,
    ) -> IResult<CompleteStr, NestedRoutes> {
        do_parse!(input,
            path_segment: apply!(path_segment, level) >>
            many1!(line_ending) >>
            routes: return_error!(
                err_str!("In routes starting here"),
                apply!(routes, level+1)
            ) >>
            (NestedRoutes { path_segment, routes })
        )
    }

    named!(pub header<CompleteStr, Header>,
        do_parse!(
            text: terminated!(
                recognize!(pair!(not!(char!('/')), many0!(not_line_ending))),
                many1!(line_ending)
            ) >>
            (Header::new(text))
        )
    );

    pub fn route_config(input: &str) -> IResult<CompleteStr, RouteConfig> {
        let input = CompleteStr(input);
        complete!(input, do_parse!(
            headers: many0!(header) >>
            many0!(line_ending) >>
            char!('/') >>
            many1!(line_ending) >>
            routes: apply!(routes, 1) >>
            ({
                let mut headers = headers;
                headers.retain(|h| h.text.len() != 0);
                RouteConfig { headers, routes }
            })
        ))
    }
}

#[cfg(test)]
mod tests {
    use nom::types::CompleteStr;

    use super::*;

    fn get_method_cases() -> Vec<(Method, &'static str)> {
        vec![
            (Method::Get, "GET"),
            (Method::Post, "POST"),
            (Method::Put, "PUT"),
            (Method::Delete, "DELETE"),
        ]
    }

    #[test]
    fn test_method_display() {
        for (method, expected) in get_method_cases().into_iter() {
            let actual = format!("{}", method);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_method_parse() {
        for (expected, text) in get_method_cases().into_iter() {
            let actual = parse::method(CompleteStr(text)).unwrap();
            assert_eq!(actual.1, expected);
            assert_eq!(actual.0, CompleteStr(""));
        }
    }

    fn get_param_cases() -> Vec<(Param, &'static str)> {
        vec![
            (Param::new("id", "Uuid"), "id: Uuid"),
            (Param::new("x", "isize"), "x: isize"),
            (Param::new("name", "String"), "name: String"),
        ]
    }

    #[test]
    fn test_param_display() {
        for (param, expected) in get_param_cases().into_iter() {
            let actual = format!("{}", param);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_param_parse() {
        for (expected, text) in get_param_cases().into_iter() {
            let actual = parse::param(CompleteStr(text)).unwrap();
            assert_eq!(actual.1, expected);
            assert_eq!(actual.0, CompleteStr(""));
        }
    }

    fn get_path_segment_static_cases() -> Vec<(PathSegment, &'static str)> {
        vec![
            (PathSegment::from("people"), "people"),
            (PathSegment::from("accounts"), "accounts"),
            (PathSegment::from("posts"), "posts"),
        ]
    }

    #[test]
    fn test_path_segment_static_display() {
        let cases = get_path_segment_static_cases();
        for (path_segment, expected) in cases.into_iter() {
            let actual = format!("{}", path_segment);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_path_segment_static_parse() {
        let cases = get_path_segment_static_cases();
        for (expected, text) in cases.into_iter() {
            let text = format!("  {}", text);
            let actual = parse::path_segment(CompleteStr(&text), 1).unwrap();
            assert_eq!(actual.1, expected);
            assert_eq!(actual.0, CompleteStr(""));
        }
    }

    #[test]
    fn test_path_segment_dynamic_display() {
        let cases = get_param_cases();
        for (param, expected) in cases.into_iter() {
            let path_segment = PathSegment::from(param);
            let actual = format!("{}", path_segment);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_path_segment_dynamic_parse() {
        let cases = get_param_cases();
        for (param, text) in cases.into_iter() {
            let text = format!("  {{{}}}", text);
            let expected = PathSegment::from(param);
            let actual = parse::path_segment(CompleteStr(&text), 1).unwrap();
            assert_eq!(actual.1, expected);
            assert_eq!(actual.0, CompleteStr(""));
        }
    }

    #[test]
    fn test_query_parameter_parse() {
        let cases = get_param_cases();
        for (expected, text) in cases.into_iter() {
            let text = format!("  [{}]", text);
            let actual = parse::query_parameter(CompleteStr(&text), 1).unwrap();
            assert_eq!(actual.1, expected);
            assert_eq!(actual.0, CompleteStr(""));
        }
    }

    fn get_resource_cases() -> Vec<(Resource, &'static str)> {
        vec![
            (
                Resource {
                    method: Method::Get,
                    name: "person".to_string(),
                    is_redirect: false,
                    query_parameters: vec![],
                },
                "GET person\n",
            ),
            (
                Resource {
                    method: Method::Get,
                    name: "person".to_string(),
                    is_redirect: true,
                    query_parameters: vec![],
                },
                "GET -> person\n",
            ),
            (
                Resource {
                    method: Method::Get,
                    name: "person".to_string(),
                    is_redirect: false,
                    query_parameters: vec![
                        Param::new("id", "usize"),
                    ],
                },
                "GET person\n  [id: usize]\n",
            ),
        ]
    }

    #[test]
    fn test_resource_stringify() {
        for (resource, expected) in get_resource_cases().into_iter() {
            let actual = resource.stringify(0);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_resource_parse() {
        for (expected, text) in get_resource_cases().into_iter() {
            let text = str::replace(text, "\n", "\n  ");
            let text = format!("  {}", text);
            let actual = parse::resource(CompleteStr(&text), 1).unwrap();
            assert_eq!(actual.1, expected);
            assert_eq!(actual.0, CompleteStr("  ")); // trailing newline
        }
    }

    fn get_routes_cases() -> Vec<(Routes, &'static str)> {
        vec![
            (
                Routes {
                    resources: vec![
                        Resource {
                            method: Method::Get,
                            name: "people".to_string(),
                            is_redirect: false,
                            query_parameters: vec![],
                        },
                        Resource {
                            method: Method::Post,
                            name: "people_new".to_string(),
                            is_redirect: false,
                            query_parameters: vec![],
                        },
                    ],
                    routes: vec![],
                    query_parameters: vec![],
                },
                "GET people\nPOST people_new\n",
            ),
            (
                Routes {
                    resources: vec![
                        Resource {
                            method: Method::Get,
                            name: "people".to_string(),
                            is_redirect: false,
                            query_parameters: vec![],
                        },
                    ],
                    routes: vec![],
                    query_parameters: vec![
                        Param::new("lang", "String"),
                    ],
                },
                "[lang: String]\nGET people\n",
            ),
            (
                Routes {
                    resources: vec![
                        Resource {
                            method: Method::Get,
                            name: "people".to_string(),
                            is_redirect: false,
                            query_parameters: vec![],
                        },
                    ],
                    routes: vec![
                        NestedRoutes {
                            path_segment: PathSegment::from(
                                Param::new("id", "Uuid"),
                            ),
                            routes: Routes {
                                resources: vec![
                                    Resource {
                                        method: Method::Get,
                                        name: "person".to_string(),
                                        is_redirect: false,
                                        query_parameters: vec![],
                                    },
                                ],
                                routes: vec![],
                                query_parameters: vec![],
                            },
                        },
                    ],
                    query_parameters: vec![],
                },
                "GET people\n{id: Uuid}\n  GET person\n",
            ),
        ]
    }

    #[test]
    fn test_routes_stringify() {
        for (routes, expected) in get_routes_cases().into_iter() {
            let actual = routes.stringify(0);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_routes_parse() {
        for (expected, text) in get_routes_cases().into_iter() {
            let text = str::replace(text, "\n", "\n  ");
            let text = format!("  {}", text);
            let actual = parse::routes(CompleteStr(&text), 1).unwrap();
            assert_eq!(actual.1, expected);
            assert_eq!(actual.0, CompleteStr("  ")); // trailing newline
        }
    }

    fn get_nested_routes_cases() -> Vec<(NestedRoutes, &'static str)> {
        vec![
            (
                NestedRoutes {
                    path_segment: PathSegment::from("people"),
                    routes: Routes {
                        resources: vec![
                            Resource {
                                method: Method::Get,
                                name: "people".to_string(),
                                is_redirect: false,
                                query_parameters: vec![],
                            },
                        ],
                        routes: vec![],
                        query_parameters: vec![],
                    },
                },
                "people\n  GET people\n",
            ),
            (
                NestedRoutes {
                    path_segment: PathSegment::from(
                        Param::new("id", "Uuid"),
                    ),
                    routes: Routes {
                        resources: vec![
                            Resource {
                                method: Method::Get,
                                name: "person".to_string(),
                                is_redirect: false,
                                query_parameters: vec![],
                            },
                            Resource {
                                method: Method::Put,
                                name: "person_save".to_string(),
                                is_redirect: false,
                                query_parameters: vec![
                                    Param::new("name", "String"),
                                ],
                            },
                        ],
                        routes: vec![],
                        query_parameters: vec![],
                    },
                },
                "{id: Uuid}\n  GET person\n  PUT person_save\n    [name: String]\n",
            ),
        ]
    }

    #[test]
    fn test_nested_routes_stringify() {
        for (nested_routes, expected) in get_nested_routes_cases().into_iter() {
            let actual = nested_routes.stringify(0);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_nested_routes_parse() {
        for (expected, text) in get_nested_routes_cases().into_iter() {
            let text = str::replace(text, "\n", "\n  ");
            let text = format!("  {}", text);
            let actual = parse::nested_routes(CompleteStr(&text), 1).unwrap();
            assert_eq!(actual.1, expected);
            assert_eq!(actual.0, CompleteStr("  ")); // trailing newline
        }
    }

    fn get_route_config_cases() -> Vec<(RouteConfig, &'static str)> {
        vec![(
            RouteConfig {
                headers: vec![
                    Header { text: "use uuid::Uuid;".to_string() },
                ],
                routes: Routes {
                    resources: vec![],
                    routes: vec![
                        NestedRoutes {
                            path_segment: PathSegment::from("people"),
                            routes: Routes {
                                resources: vec![
                                    Resource {
                                        method: Method::Get,
                                        name: "people".to_string(),
                                        is_redirect: false,
                                        query_parameters: vec![],
                                    },
                                ],
                                routes: vec![
                                    NestedRoutes {
                                        path_segment: PathSegment::from(
                                            Param::new("id", "Uuid"),
                                        ),
                                        routes: Routes {
                                            resources: vec![
                                                Resource {
                                                    method: Method::Get,
                                                    name: "person".to_string(),
                                                    is_redirect: false,
                                                    query_parameters: vec![],
                                                },
                                                Resource {
                                                    method: Method::Put,
                                                    name: "person_save".to_string(),
                                                    is_redirect: false,
                                                    query_parameters: vec![
                                                        Param::new("name", "String"),
                                                    ],
                                                },
                                            ],
                                            routes: vec![],
                                            query_parameters: vec![],
                                        },
                                    },
                                ],
                                query_parameters: vec![],
                            },
                        },
                    ],
                    query_parameters: vec![
                        Param::new("lang", "String"),
                    ],
                },
            },
            "use uuid::Uuid;

/
  [lang: String]
  people
    GET people
    {id: Uuid}
      GET person
      PUT person_save
        [name: String]
",
        )]
    }

    #[test]
    fn test_route_config_stringify() {
        for (route_config, expected) in get_route_config_cases().into_iter() {
            let actual = route_config.stringify();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_route_config_parse() {
        for (expected, text) in get_route_config_cases().into_iter() {
            let actual = parse::route_config(text).unwrap().1;
            assert_eq!(actual, expected);
        }
    }
}
