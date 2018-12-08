use std::io;

use crate::config::RouteConfig;
use crate::trie::Trie;
use crate::flat::{Charlike, FlattenedRoute, FlattenedRoutes};

pub fn to_caps_case(s: &str) -> String {
    let mut chars = s.chars();
    let mut res = String::new();

    let mut ch = match chars.next() {
        None => return res,
        Some(c) => Some(c.to_ascii_uppercase()),
    };

    while ch.is_some() {
        match ch.unwrap() {
            '_' => {
                match chars.next() {
                    None => return res,
                    Some(c) => res.push(c.to_ascii_uppercase()),
                }
            },
            c => res.push(c),
        }

        ch = chars.next()
    }

    res
}

pub fn codegen(
    w: &mut io::Write,
    route_config: &RouteConfig,
) -> io::Result<()> {
    let flattened = FlattenedRoutes::from(&route_config.routes);


    writeln!(w, "pub mod routes {{")?;
    writeln!(w)?;
    writeln!(w, "//! Application route configuration.")?;
    writeln!(w, "//!")?;
    writeln!(w, "//! Of note is the function [`match_route`] as well as request structs")?;
    writeln!(w, "//! specific to each named resource.")?;
    writeln!(w, "//!")?;
    writeln!(w, "//! Route configuration:")?;
    writeln!(w, "//!")?;
    writeln!(w, "//!     /")?;

    let stringified_config = str::replace(
        &route_config.routes.stringify(1),
        "\n",
        "\n//!     ",
    );
    writeln!(w, "//!     {}", stringified_config);

    writeln!(w, "//! [`match_route`]: fn.match_route.html")?;
    writeln!(w)?;
    writeln!(w, "#![allow(unused_imports)]")?;
    writeln!(w, "#![allow(dead_code)]")?;
    writeln!(w)?;

    for header in route_config.headers.iter() {
        writeln!(w, "{}", header.text)?;
    }

    if route_config.headers.len() > 0 {
        writeln!(w)?;
    }

    for route in flattened.iter() {
        for resource in route.resources.iter() {
            if resource.is_redirect { continue }


            writeln!(w, "/// Parameters for a request to the {} route.", resource.name)?;
            writeln!(w, "#[derive(Debug)]")?;
            writeln!(w, "pub struct {}Params {{", to_caps_case(&resource.name))?;

            for param in route.path.dynamics() {
                writeln!(w, "    {}: {},", param.name, param.typ)?;
            }
            for param in route.query_parameters.iter() {
                writeln!(w, "    {}: Option<{}>,", param.name, param.typ)?;
            }
            for param in resource.query_parameters.iter() {
                writeln!(w, "    {}: Option<{}>,", param.name, param.typ)?;
            }

            writeln!(w, "}}")?;
            writeln!(w)?;
            writeln!(w, "impl {}Params {{", to_caps_case(&resource.name))?;
            writeln!(w, "    /// Make a path to this resource with the given parameters.")?;
            writeln!(w, "    pub fn to_path(&self) -> String {{")?;
            writeln!(w, "        #[allow(unused_mut)]")?;
            write!(w, "        let mut s = String::from(\"/")?;

            let mut path = route.path.iter().peekable();
            loop {
                let ch = match path.next() {
                    None => break,
                    Some(c) => c.clone(),
                };
                match ch {
                    Charlike::Static(s) => {
                        write!(w, "{}", s)?;
                    },
                    Charlike::Dynamic(ref p) => {
                        writeln!(w, "\");")?;
                        writeln!(w, "        let text = format!(\"{{}}\", self.{});", p)?;
                        writeln!(w, "        s.push_str(&text);")?;
                        write!(w, "        s.push_str(\"")?;
                    },
                    Charlike::Separator => {
                        match path.peek() {
                            None => {},
                            Some(_) => {
                                write!(w, "/")?;
                            },
                        }
                    },
                }
            }

            writeln!(w, "\");")?;
            writeln!(w, "        s")?;
            writeln!(w, "    }}")?;
            writeln!(w, "}}")?;
            writeln!(w)?;

        }
    }

    writeln!(w, "/// An active route in the application -- match against this.")?;
    writeln!(w, "#[derive(Debug)]")?;
    writeln!(w, "pub enum Route {{")?;

    for route in flattened.iter() {
        for resource in route.resources.iter() {
            if resource.is_redirect { continue }

            writeln!(w, "    {0}({0}Params),", to_caps_case(&resource.name))?;
        }
    }

    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(w, "impl Route {{")?;
    writeln!(w, "    /// Make a path to this route with the given parameters.")?;
    writeln!(w, "    pub fn to_path(&self) -> String {{")?;
    writeln!(w, "        match self {{")?;

    for route in flattened.iter() {
        for resource in route.resources.iter() {
            if resource.is_redirect { continue }

            writeln!(w, "            Route::{}(p) => p.to_path(),", to_caps_case(&resource.name))?;
        }
    }

    writeln!(w, "        }}")?;
    writeln!(w, "    }}")?;
    writeln!(w, "}}")?;
    writeln!(w)?;

    writeln!(w, "/// Match an incoming request against this router.")?;
    writeln!(w, "///")?;
    writeln!(w, "/// Accepts an iterator for the characters of the request path,")?;
    writeln!(w, "/// as well as a [`wayfinder::Method`] for the HTTP verb.")?;
    writeln!(w, "/// Returns a `Result`, usually `Ok` with the result of the")?;
    writeln!(w, "/// [`wayfinder::Match`].")?;
    writeln!(w, "///")?;
    writeln!(w, "/// If the match was successful, it will be a `Match::Route` or")?;
    writeln!(w, "/// `Match::Redirect` with the parameters enclosed.  You can then")?;
    writeln!(w, "/// match on the [`Route`] to pass control of the request along to")?;
    writeln!(w, "/// a specific handler.")?;
    writeln!(w, "///")?;
    writeln!(w, "/// If there is no match, this will return `Match::NotFound`")?;
    writeln!(w, "/// if no path matches (which you could return as `404 Not Found`),")?;
    writeln!(w, "/// or `Match::NotAllowed` if no method matches (in which case a")?;
    writeln!(w, "/// `405 Not Allowed` would be appropriate).")?;
    writeln!(w, "///")?;
    writeln!(w, "/// If a route parameter fails to parse correctly, this will return")?;
    writeln!(w, "/// `Err` with the underlying parsing error.  Usually you'll want")?;
    writeln!(w, "/// to send back a `400 Bad Request` for that.")?;
    writeln!(w, "///")?;
    // TODO: these relative paths assume way too much
    // TODO: make these point to the specific version on docs.rs
    writeln!(w, "/// [`wayfinder::Method`]: ../../wayfinder/enum.Method.html")?;
    writeln!(w, "/// [`wayfinder::Match`]: ../../wayfinder/enum.Match.html")?;
    writeln!(w, "/// [`Route`]: enum.Route.html")?;

    writeln!(w, "pub fn match_route<P: std::iter::Iterator<Item=char>>(")?;
    writeln!(w, "    path: &mut P,")?;
    writeln!(w, "    method: wayfinder::Method,")?;
    writeln!(w, ") -> Result<wayfinder::Match<Route>, wayfinder::Error> {{")?;
    writeln!(w, "    use wayfinder::{{Error, Method, Match}};")?;
    writeln!(w, "    let mut path = path.fuse().peekable();")?;

    writeln!(w, "    if path.peek() == Some(&'/') {{")?;
    writeln!(w, "        path.next();")?;
    writeln!(w, "    }}")?;

    codegen_trie(w, &flattened.to_trie(), 1)?;

    writeln!(w, "}}")?;
    writeln!(w)?;
    writeln!(w, "}} // mod routes")?;

    Ok(())
}

pub fn codegen_trie(
    w: &mut io::Write,
    trie: &Trie<Charlike, FlattenedRoute>,
    indent: usize,
) -> io::Result<()> {
    let mut indent1 = String::new();
    for _ in 0..indent { indent1.push_str("    "); }
    let mut indent2 = indent1.clone();
    indent2.push_str("    ");

    let has_dynamic = trie.children.iter().any(|c| match c.0 {
        Charlike::Dynamic(_) => true,
        _ => false,
    });

    if has_dynamic {
        writeln!(w);
        writeln!(w, "{}let mut text = String::new();", indent1)?;
        writeln!(w);
    }

    writeln!(w, "{}match path.next() {{", indent1)?;

    let mut wrote_none = false;
    match trie.data {
        Some(ref route) if route.resources.len() != 0 => {
            write_methods(w, route, indent)?;
            wrote_none = true;
        },
        _ => {},
    }

    if trie.children.len() == 1 {
        match trie.children[0].0 {
            Charlike::Static(ref c) => {
                writeln!(w, "{}Some('{}') => {{}},", indent2, c)?;
            },
            Charlike::Dynamic(ref p) => {
                if !wrote_none {
                    writeln!(w, "{}None => return Ok(Match::NotFound),", indent2)?;
                }
                write_dynamic(w, &trie.children[0].1, indent, p)?;
            },
            Charlike::Separator => {
                if let Some(ref route) = trie.children[0].1.data {
                    if route.resources.len() != 0 {
                        write_methods(w, route, indent)?;
                    }
                }
                writeln!(w, "{}Some('/') => {{}}", indent2)?;
            }
        }
    } else { // trie.children.len() > 1
        for child in trie.children.iter() {
            match child.0 {
                Charlike::Static(c) => {
                    writeln!(w, "{}Some('{}') => {{", indent2, c)?;

                    codegen_trie(w, &child.1, indent+2)?;

                    writeln!(w, "{}}},", indent2)?;
                },
                Charlike::Dynamic(ref p) => {
                    if !wrote_none {
                        writeln!(w, "{}None => return Ok(Match::NotFound),", indent2)?;
                    }
                    write_dynamic(w, &child.1, indent, p)?;
                    // No further routes will possibly match.
                    break;
                },
                Charlike::Separator => {
                    writeln!(w, "{}Some('/') => {{}}", indent2)?;
                }
            }
        }
    }
    if !has_dynamic {
        writeln!(w, "{}_ => return Ok(Match::NotFound),", indent2)?;
        writeln!(w, "{}}}", indent1)?;
    }

    if trie.children.len() == 1 {
        codegen_trie(w, &trie.children[0].1, indent)?;
    }

    Ok(())
}

fn write_methods(
    w: &mut io::Write,
    route: &FlattenedRoute,
    indent: usize,
) -> io::Result<()> {
    let mut indent1 = String::from("    ");
    for _ in 0..indent { indent1.push_str("    "); }

    writeln!(w, "{}None => match method {{", indent1)?;

    for resource in route.resources.iter() {
        writeln!(w, "{}    Method::{:?} => return Ok(Match::{}(Route::{3}({3}Params {{",
            indent1,
            resource.method,
            if resource.is_redirect { "Redirect" } else { "Route" },
            to_caps_case(&resource.name),
        )?;

            for param in route.path.dynamics() {
                writeln!(w, "{}        {},", indent1, param.name)?;
            }
            for param in route.query_parameters.iter() {
                writeln!(w, "{}        {}: None,", indent1, param.name)?;
            }
            for param in resource.query_parameters.iter() {
                writeln!(w, "{}        {}: None,", indent1, param.name)?;
            }

        writeln!(w, "{}    }})))," , indent1)?;
    }

    writeln!(w, "{}    _ => return Ok(Match::NotAllowed),", indent1)?;
    writeln!(w, "{}}},", indent1)?;

    Ok(())
}

// assumes that a None case has already been written
fn write_dynamic(
    w: &mut io::Write,
    mut trie: &Trie<Charlike, FlattenedRoute>,
    indent: usize,
    name: &str,
) -> io::Result<()> {
    let mut indent1 = String::new();
    for _ in 0..indent { indent1.push_str("    "); }
    let mut indent2 = indent1.clone();
    indent2.push_str("    ");

    writeln!(w, "{}Some(c) => text.push(c),", indent2)?;
    writeln!(w, "{}}}", indent1)?;
    writeln!(w);
    writeln!(w, "{}loop {{", indent1)?;

    writeln!(w, "{}match path.peek().cloned() {{", indent2)?;
    writeln!(w, "{}    None => break,", indent2)?;
    writeln!(w, "{}    Some(c) => {{", indent2)?;
    writeln!(w, "{}        path.next();", indent2)?;
    writeln!(w, "{}        if c == '/' {{", indent2)?;
    writeln!(w, "{}            break;", indent2)?;
    writeln!(w, "{}        }} else {{", indent2)?;
    writeln!(w, "{}            text.push(c);", indent2)?;
    writeln!(w, "{}        }}", indent2)?;
    writeln!(w, "{}    }},", indent2)?;
    writeln!(w, "{}}}", indent2)?;

    writeln!(w, "{}}};", indent1)?;
    writeln!(w);
    writeln!(w, "{}let {} = text.parse()", indent1, name)?;
    writeln!(w, "{}    .map_err(|e| Error::fail(\"{}\", e))?;", indent1, name)?;
    writeln!(w);

    if trie.children.len() != 1 {
        return Err(io::ErrorKind::InvalidInput.into());
    }

    trie = &trie.children[0].1;

    codegen_trie(w, trie, indent)?;

    Ok(())
}
