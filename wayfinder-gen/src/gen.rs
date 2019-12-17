use std::io;
use std::io::Write;

use crate::flat::{Charlike, FlattenedModule, FlattenedModules, FlattenedRoute, FlattenedRoutes};
use crate::trie::Trie;
use wayfinder_core::{Header, RouteConfig};

pub fn to_caps_case(s: &str) -> String {
    let mut chars = s.chars();
    let mut res = String::new();

    let mut ch = match chars.next() {
        None => return res,
        Some(c) => Some(c.to_ascii_uppercase()),
    };

    while ch.is_some() {
        match ch.unwrap() {
            '_' => match chars.next() {
                None => return res,
                Some(c) => res.push(c.to_ascii_uppercase()),
            },
            c => res.push(c),
        }

        ch = chars.next()
    }

    res
}

pub fn to_snake_case(s: &str) -> String {
    let mut chars = s.chars();
    let mut res = String::new();

    let mut ch = match chars.next() {
        None => return res,
        Some(c) => Some(c.to_ascii_lowercase()),
    };

    while ch.is_some() {
        let c = ch.unwrap();

        if c.is_ascii_uppercase() {
            res.push('_');
            res.push(c.to_ascii_lowercase());
        } else {
            res.push(c);
        }

        ch = chars.next()
    }

    res
}

pub fn codegen_module<W>(
    w: &mut W,
    module: &FlattenedModule,
    headers: &[Header],
    indent: &str,
) -> io::Result<()>
where
    W: Write,
{
    let is_root = indent == "    ";

    // TODO: can we do something smarter than repeating these everywhere?
    for header in headers.iter() {
        writeln!(w, "{}{}", indent, header.text)?;
    }

    if headers.len() > 0 {
        writeln!(w)?;
    }

    for action in module.actions.iter() {
        write!(w, "{}/// Renders for `{} /", indent, action.method)?;

        let mut path = action.path.iter().peekable();
        loop {
            let ch = match path.next() {
                None => break,
                Some(c) => c.clone(),
            };
            match ch {
                Charlike::Static(s) => {
                    write!(w, "{}", s)?;
                }
                Charlike::Dynamic(ref p) => {
                    write!(w, "{{{}}}", p)?;
                }
                Charlike::Separator => match path.peek() {
                    None => {}
                    Some(_) => {
                        write!(w, "/")?;
                    }
                },
            }
        }

        writeln!(w, "`.")?;

        writeln!(w, "{}#[derive(Debug, PartialEq, Eq)]", indent)?;
        write!(w, "{}pub struct {}", indent, to_caps_case(&action.name))?;

        if action.route_parameters.is_empty() && action.query_parameters.is_empty() {
            writeln!(w, ";")?;
        } else {
            writeln!(w, " {{")?;

            for param in action.route_parameters.iter() {
                writeln!(w, "{}    pub {}: {},", indent, param.name, param.typ)?;
            }
            for param in action.query_parameters.iter() {
                writeln!(
                    w,
                    "{}    pub {}: Option<{}>,",
                    indent, param.name, param.typ
                )?;
            }

            writeln!(w, "{}}}", indent)?;
        }

        writeln!(w)?;

        writeln!(w, "{}impl {} {{", indent, to_caps_case(&action.name))?;
        writeln!(
            w,
            "{}    /// Make a path to this route with the given parameters.",
            indent
        )?;
        writeln!(w, "{}    pub fn to_path(&self) -> String {{", indent)?;

        if !action.route_parameters.is_empty() || !action.query_parameters.is_empty() {
            write!(
                w,
                "{}        let {} {{ ",
                indent,
                to_caps_case(&action.name)
            )?;

            for param in action.route_parameters.iter() {
                write!(w, "ref {}, ", param.name)?;
            }

            for param in action.query_parameters.iter() {
                write!(w, "ref {}, ", param.name)?;
            }

            writeln!(w, "}} = self;")?;
        }

        write!(w, "{}        format!(\"/", indent)?;

        let mut path = action.path.iter().peekable();
        loop {
            let ch = match path.next() {
                None => break,
                Some(c) => c.clone(),
            };
            match ch {
                Charlike::Static(s) => {
                    write!(w, "{}", s)?;
                }
                Charlike::Dynamic(_) => {
                    write!(w, "{{}}")?;
                }
                Charlike::Separator => match path.peek() {
                    None => {}
                    Some(_) => {
                        write!(w, "/")?;
                    }
                },
            }
        }

        write!(w, "\"")?;

        for param in action.route_parameters.iter() {
            write!(w, ", {}", param.name)?;
        }

        writeln!(w, ")")?;
        writeln!(w, "{}    }}", indent)?;
        writeln!(w, "{}}}", indent)?;

        writeln!(w)?;
    }

    for module in module.modules.iter() {
        writeln!(w, "{}pub mod {} {{", indent, to_snake_case(&module.name))?;

        codegen_module(w, &module, &headers, &format!("{}    ", indent))?;

        writeln!(w, "{}}}", indent)?;
        writeln!(w)?;
    }

    if is_root {
        writeln!(
            w,
            "{}/// An active route in the application -- match against this.",
            indent
        )?;
    } else {
        writeln!(
            w,
            "{}/// Parameters for requests to the {} controller.",
            indent,
            to_snake_case(&module.name)
        )?;
    }
    writeln!(w, "{}#[derive(Debug, PartialEq, Eq)]", indent)?;
    writeln!(w, "{}pub enum Route {{", indent)?;

    for action in module.actions.iter() {
        writeln!(
            w,
            "{}    {}({}),",
            indent,
            to_caps_case(&action.name),
            to_caps_case(&action.name)
        )?;
    }

    for module in module.modules.iter() {
        writeln!(
            w,
            "{}    {}({}::Route),",
            indent,
            to_caps_case(&module.name),
            to_snake_case(&module.name)
        )?;
    }

    writeln!(w, "{}}}", indent)?;

    writeln!(w)?;

    writeln!(w, "{}impl Route {{", indent)?;
    writeln!(
        w,
        "{}    /// Make a path to this route with the given parameters.",
        indent
    )?;
    writeln!(w, "{}    pub fn to_path(&self) -> String {{", indent)?;
    writeln!(w, "{}        match self {{", indent)?;

    for action in module.actions.iter() {
        writeln!(
            w,
            "{}            Route::{}(ref route) => route.to_path(),",
            indent,
            to_caps_case(&action.name)
        )?;
    }

    for module in module.modules.iter() {
        writeln!(
            w,
            "{}            Route::{}(ref route) => route.to_path(),",
            indent,
            to_caps_case(&module.name)
        )?;
    }

    writeln!(w, "{}        }}", indent)?;
    writeln!(w, "{}    }}", indent)?;
    writeln!(w, "{}}}", indent)?;

    Ok(())
}

pub fn codegen<W>(w: &mut W, route_config: &RouteConfig) -> io::Result<()>
where
    W: Write,
{
    let flattened = FlattenedRoutes::from(&route_config.routes);
    let modules = FlattenedModules::from(&route_config.routes);

    writeln!(w, "pub mod routes {{")?;
    writeln!(w)?;
    writeln!(w, "    //! Application route configuration.")?;
    writeln!(w, "    //!")?;
    writeln!(
        w,
        "    //! Of note is the function [`match_route`] as well as request structs"
    )?;
    writeln!(w, "    //! specific to each named resource.")?;
    writeln!(w, "    //!")?;
    writeln!(w, "    //! Route configuration:")?;
    writeln!(w, "    //!")?;
    writeln!(w, "    //! ```ignore")?;
    writeln!(w, "    //! /")?;

    let stringified_config = str::replace(&route_config.routes.stringify(1), "\n", "\n    //! ");
    let stringified_config = &stringified_config[..stringified_config.len() - 8];
    write!(w, "    //! {}", stringified_config)?;
    writeln!(w, "    //! ```")?;
    writeln!(w, "    //!")?;

    writeln!(w, "    //! [`match_route`]: fn.match_route.html")?;
    writeln!(w)?;
    writeln!(w, "    #![allow(dead_code)]")?;
    writeln!(w, "    #![allow(unused_imports)]")?;
    writeln!(w, "    #![allow(unused_mut)]")?;
    writeln!(w, "    #![allow(unused_variables)]")?;
    writeln!(w)?;

    codegen_module(w, &modules.root, &route_config.headers, "    ")?;
    writeln!(w)?;

    writeln!(w, "    #[derive(PartialEq, Eq)]")?;
    writeln!(w, "    pub enum Match<T> {{")?;
    writeln!(w, "        NotFound,")?;
    writeln!(w, "        NotAllowed,")?;
    writeln!(w, "        Route(T),")?;
    writeln!(w, "        Redirect(T),")?;
    writeln!(w, "    }}")?;
    writeln!(w)?;
    writeln!(w, "    use std::fmt;")?;
    writeln!(w, "    impl<T: fmt::Debug> fmt::Debug for Match<T> {{")?;
    writeln!(w, "        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(w, "            match self {{")?;
    writeln!(w, "                Match::NotFound => write!(f, \"Match::NotFound\"),")?;
    writeln!(w, "                Match::NotAllowed => write!(f, \"Match::NotAllowed\"),")?;
    writeln!(w, "                Match::Route(t) => write!(f, \"Match::Route({{:?}})\", t),")?;
    writeln!(w, "                Match::Redirect(t) => write!(f, \"Match::Redirect({{:?}})\", t),")?;
    writeln!(w, "            }}")?;
    writeln!(w, "        }}")?;
    writeln!(w, "    }}")?;
    writeln!(w)?;
    writeln!(w, "    pub struct Error {{")?;
    writeln!(w, "        param: String,")?;
    writeln!(w, "        what: Box<dyn fmt::Debug>,")?;
    writeln!(w, "    }}")?;
    writeln!(w)?;
    writeln!(w, "    impl fmt::Debug for Error {{")?;
    writeln!(w, "        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(w, "            f.debug_struct(\"wayfinder::Error\")")?;
    writeln!(w, "                .field(\"param\", &self.param)")?;
    writeln!(w, "                .field(\"what\", &self.what)")?;
    writeln!(w, "                .finish()")?;
    writeln!(w, "        }}")?;
    writeln!(w, "    }}")?;
    writeln!(w)?;
    writeln!(w, "    impl fmt::Display for Error {{")?;
    writeln!(w, "        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
    writeln!(w, "            write!(")?;
    writeln!(w, "                f,")?;
    writeln!(w, "                \"Error parsing '{{}}' parameter {{:?}}\",")?;
    writeln!(w, "                self.param, self.what")?;
    writeln!(w, "            )")?;
    writeln!(w, "        }}")?;
    writeln!(w, "    }}")?;
    writeln!(w)?;
    writeln!(w, "    impl std::error::Error for Error {{}}")?;
    writeln!(w)?;
    writeln!(w, "    impl Error {{")?;
    writeln!(w, "        pub fn fail<S: AsRef<str>, T: fmt::Debug + 'static>(param: S, what: T) -> Error {{")?;
    writeln!(w, "            Error {{")?;
    writeln!(w, "                param: param.as_ref().to_string(),")?;
    writeln!(w, "                what: Box::new(what),")?;
    writeln!(w, "            }}")?;
    writeln!(w, "        }}")?;
    writeln!(w, "    }}")?;

    writeln!(w, "    /// Match a path and method against this router.")?;
    writeln!(w, "    ///")?;
    writeln!(
        w,
        "    /// Accepts an iterator for the characters of the request path,"
    )?;
    writeln!(
        w,
        "    /// as well as a [`wayfinder::Method`] for the HTTP verb."
    )?;
    writeln!(
        w,
        "    /// Returns a `Result`, usually `Ok` with the result of the"
    )?;
    writeln!(w, "    /// [`wayfinder::Match`].")?;
    writeln!(w, "    ///")?;
    writeln!(
        w,
        "    /// If the match was successful, it will be a `Match::Route` or"
    )?;
    writeln!(
        w,
        "    /// `Match::Redirect` with the parameters enclosed.  You can then"
    )?;
    writeln!(
        w,
        "    /// match on the [`Route`] to pass control of the request along to"
    )?;
    writeln!(w, "    /// a specific handler.")?;
    writeln!(w, "    ///")?;
    writeln!(
        w,
        "    /// If there is no match, this will return `Match::NotFound`"
    )?;
    writeln!(
        w,
        "    /// if no path matches (which you could return as `404 Not Found`),"
    )?;
    writeln!(
        w,
        "    /// or `Match::NotAllowed` if no method matches (in which case a"
    )?;
    writeln!(w, "    /// `405 Not Allowed` would be appropriate).")?;
    writeln!(w, "    ///")?;
    writeln!(
        w,
        "    /// If a route parameter fails to parse correctly, this will return"
    )?;
    writeln!(
        w,
        "    /// `Err` with the underlying parsing error.  Usually you'll want"
    )?;
    writeln!(w, "    /// to send back a `400 Bad Request` for that.")?;
    writeln!(w, "    ///")?;
    // TODO: these relative paths assume way too much
    // TODO: make these point to the specific version on docs.rs
    writeln!(
        w,
        "    /// [`wayfinder::Method`]: ../../wayfinder/enum.Method.html"
    )?;
    writeln!(
        w,
        "    /// [`wayfinder::Match`]: ../../wayfinder/enum.Match.html"
    )?;
    writeln!(w, "    /// [`Route`]: enum.Route.html")?;

    writeln!(w, "    pub fn match_route<P: AsRef<[u8]>, M: AsRef<[u8]>>(")?;
    writeln!(w, "        path: P,")?;
    writeln!(w, "        method: M,")?;
    writeln!(
        w,
        "    ) -> Result<Match<Route>, Error> {{"
    )?;
    writeln!(w, "        let method = method.as_ref();")?;
    writeln!(w, "        let path = path.as_ref();")?;
    writeln!(w, "        let len = path.len();")?;
    writeln!(
        w,
        "        let mut i = if len > 0 && &path[0..1] == b\"/\" {{ 1 }} else {{ 0 }};"
    )?;

    writeln!(w)?;

    codegen_trie(w, &flattened.to_trie(), 2)?;

    writeln!(w, "    }}")?;
    writeln!(w)?;
    writeln!(w, "}} // mod routes")?;

    Ok(())
}

pub fn codegen_trie<W>(
    w: &mut W,
    trie: &Trie<Charlike, FlattenedRoute>,
    indent: usize,
) -> io::Result<()>
where
    W: Write,
{
    let mut indent1 = String::new();
    for _ in 0..indent {
        indent1.push_str("    ");
    }
    let mut indent2 = indent1.clone();
    indent2.push_str("    ");

    if let Some(ref route) = trie.data {
        if route.resources.len() != 0 {
            write_methods(w, route, indent)?;
        }
    }

    if trie.children.len() == 0 {
        writeln!(w, "{}return Ok(Match::NotFound);", indent1)?;
        return Ok(());
    }

    if trie.children.len() == 1 {
        let (ref segment, ref child) = trie.children[0];
        match segment {
            Charlike::Separator => {
                // check for child match
                // TODO: this seems backwards??
                match child.data {
                    Some(ref route) if route.resources.len() != 0 => {
                        write_methods(w, route, indent)?;
                    }
                    _ => {
                        writeln!(w, "{}if i == len {{", indent1)?;
                        writeln!(w, "{}    return Ok(Match::NotFound);", indent1)?;
                        writeln!(w, "{}}}", indent1)?;
                    }
                }

                // check for separator
                writeln!(w, "{}match &path[i..i+1] {{", indent1)?;
                writeln!(w, "{}    b\"/\" => {{", indent1)?;
                writeln!(w, "{}        i += 1;", indent1)?;
                writeln!(w, "{}    }},", indent1)?;
                writeln!(w, "{}    _ => return Ok(Match::NotFound),", indent1)?;
                writeln!(w, "{}}}", indent1)?;

                // continue with child
                codegen_trie(w, child, indent)?;
            }
            Charlike::Static(ch) => {
                // find unambiguous match
                let mut unambiguous = String::new();
                unambiguous.push(*ch);

                let mut child = child;

                loop {
                    if child.children.len() == 1 {
                        if let Charlike::Static(ch) = child.children[0].0 {
                            unambiguous.push(ch);
                            child = &child.children[0].1;
                            continue;
                        }
                    }
                    break;
                }

                let match_len = unambiguous.len();
                if match_len == 1 {
                    writeln!(w, "{}if i == len {{", indent1)?;
                } else {
                    writeln!(w, "{}if i + {} > len {{", indent1, match_len)?;
                }
                writeln!(w, "{}    return Ok(Match::NotFound);", indent1)?;
                writeln!(w, "{}}}", indent1)?;

                // check it
                writeln!(w, "{}match &path[i..i+{}] {{", indent1, match_len)?;
                writeln!(w, "{}    b\"{}\" => {{", indent1, unambiguous)?; // TODO quotes in paths????
                writeln!(w, "{}        i += {};", indent1, match_len)?;
                writeln!(w, "{}    }},", indent1)?;
                writeln!(w, "{}    _ => return Ok(Match::NotFound),", indent1)?;
                writeln!(w, "{}}}", indent1)?;

                // continue after unambiguous
                codegen_trie(w, child, indent)?;
            }
            Charlike::Dynamic(ref name) => {
                writeln!(w, "{}let start = i;", indent1)?;
                write_dynamic(w, &child, indent, name)?;
            }
        }

        return Ok(());
    }

    // n.b. if we got here, trie.children.len() > 1

    let dynamic = trie.children.iter().find(|c| match c.0 {
        Charlike::Dynamic(_) => true,
        _ => false,
    });
    let has_dynamic = match dynamic {
        Some(_) => true,
        None => false,
    };

    if has_dynamic {
        writeln!(w)?;
        writeln!(w, "{}let start = i;", indent1)?;
        writeln!(w)?;
    }

    let (unambiguous, l, next) = {
        let mut l = 0;
        let mut s = String::new();
        let mut t = trie;

        loop {
            let has_separator = t.children.iter().any(|c| match c.0 {
                Charlike::Separator => true,
                _ => false,
            });
            if has_separator {
                break;
            }

            l += 1;

            let options = t
                .children
                .iter()
                .filter_map(|c| match c.0 {
                    Charlike::Dynamic(_) => None,
                    Charlike::Separator => unreachable!(),
                    Charlike::Static(ch) => Some((ch, &c.1)),
                })
                .collect::<Vec<_>>();

            if options.len() != 1 {
                break;
            }

            s.push(options[0].0);
            t = options[0].1;
        }

        (s, l, t)
    };

    let match_len = if l == 0 { 1 } else { l };

    if unambiguous != "" {
        // n.b. if we got here, the next bit is unambiguous save a dynamic

        // TODO: refactor code to make this clearer!
        assert!(
            has_dynamic,
            "there must be a dynamic or we did something wrong"
        );

        if match_len == 1 {
            writeln!(w, "{}if i <= len {{", indent1)?;
        } else {
            writeln!(w, "{}if i + {} <= len {{", indent1, match_len)?;
        }

        writeln!(w, "{}match &path[i..i+{}] {{", indent2, match_len)?;
        writeln!(w, "{}    b\"{}\" => {{", indent2, unambiguous)?; // TODO: quotes in paths??
        writeln!(w, "{}        i += {};", indent2, match_len)?;

        codegen_trie(w, next, indent + 3)?;

        writeln!(w, "{}    }},", indent2)?;

        writeln!(w, "{}    _ => {{}},", indent2)?;
        writeln!(w, "{}}}", indent2)?;
        writeln!(w, "{}}}", indent1)?;

        if let Charlike::Dynamic(ref name) = dynamic.unwrap().0 {
            write_dynamic(w, &dynamic.unwrap().1, indent, name)?;
        } else {
            unreachable!();
        }

        return Ok(());
    }

    // n.b. if we got here, the next character is ambiguous
    writeln!(w, "{}if i == len {{", indent1)?;
    writeln!(w, "{}    return Ok(Match::NotFound);", indent1)?;
    writeln!(w, "{}}}", indent1)?;

    writeln!(w, "{}match &path[i..i+1] {{", indent1)?;

    for child in trie.children.iter() {
        match child.0 {
            Charlike::Static(c) => {
                writeln!(w, "{}b\"{}\" => {{", indent2, c)?;
                writeln!(w, "{}    i += 1;", indent2)?;

                codegen_trie(w, &child.1, indent + 2)?;

                writeln!(w, "{}}},", indent2)?;
            }
            Charlike::Dynamic(ref p) => {
                write_dynamic(w, &child.1, indent, p)?;

                // TODO: is this true still?
                // No further routes will possibly match.
                break;
            }
            Charlike::Separator => {
                writeln!(w, "{}Some('/') => {{}}", indent2)?;
            }
        }
    }

    writeln!(w, "{}    _ => return Ok(Match::NotFound),", indent1)?;

    writeln!(w, "{}}}", indent1)?;

    Ok(())
}

fn write_methods<W>(w: &mut W, route: &FlattenedRoute, indent: usize) -> io::Result<()>
where
    W: Write,
{
    let mut indent1 = String::from("");
    for _ in 0..indent {
        indent1.push_str("    ");
    }

    writeln!(w, "{}if i == len {{", indent1)?;
    writeln!(w, "{}    match method {{", indent1)?;

    for resource in route.resources.iter() {
        let (path, route_nest, close_parens) = {
            let mut path = String::new();
            let mut accum = String::new();
            let mut parens = String::new();
            for module in resource.modules.iter() {
                accum.push_str(&format!("{}Route::{}(", path, to_caps_case(module)));
                path.push_str(&format!("{}::", to_snake_case(module)));
                parens.push(')');
            }
            (path, accum, parens)
        };

        writeln!(
            w,
            "{0}        {1} => return Ok(Match::{2}({3}{4}Route::{5}({4}{5} {{",
            indent1,
            resource.method.byte_str(),
            if resource.is_redirect {
                "Redirect"
            } else {
                "Route"
            },
            route_nest,
            path,
            to_caps_case(&resource.name),
        )?;

        for param in route.path.dynamics() {
            writeln!(w, "{}            {},", indent1, param.name)?;
        }
        for param in route.query_parameters.iter() {
            writeln!(w, "{}            {}: None,", indent1, param.name)?;
        }
        for param in resource.query_parameters.iter() {
            writeln!(w, "{}            {}: None,", indent1, param.name)?;
        }

        writeln!(w, "{}        }}{}))),", indent1, close_parens)?;
    }

    writeln!(w, "{}        _ => return Ok(Match::NotAllowed),", indent1)?;
    writeln!(w, "{}    }}", indent1)?;
    writeln!(w, "{}}}", indent1)?;

    Ok(())
}

// assumes that a None case has already been written
fn write_dynamic<W>(
    w: &mut W,
    trie: &Trie<Charlike, FlattenedRoute>,
    indent: usize,
    name: &str,
) -> io::Result<()>
where
    W: Write,
{
    let mut indent1 = String::new();
    for _ in 0..indent {
        indent1.push_str("    ");
    }
    let mut indent2 = indent1.clone();
    indent2.push_str("    ");

    writeln!(w)?;
    writeln!(w, "{}while i < len && &path[i..i+1] != b\"/\" {{", indent1)?;
    writeln!(w, "{}    i += 1;", indent1)?;
    writeln!(w, "{}}}", indent1)?;
    writeln!(w)?;
    writeln!(
        w,
        "{}let text = std::str::from_utf8(&path[start..i]).unwrap();",
        indent1
    )?;
    writeln!(w, "{}let {} = text.parse()", indent1, name)?;
    writeln!(
        w,
        "{}    .map_err(|e| Error::fail(\"{}\", e))?;",
        indent1, name
    )?;
    writeln!(w)?;

    // must be followed by a separator
    if trie.children.len() != 1 {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    if trie.children[0].0 != Charlike::Separator {
        return Err(io::ErrorKind::InvalidInput.into());
    }

    codegen_trie(w, trie, indent)?;

    Ok(())
}
