// TODO: this parser is functional, but there's a lot more that can
// be done to improve robustness and error reporting.

use lazy_static::lazy_static;
use nom::types::CompleteStr;
use nom::{
    alt_complete, apply, char, complete, count, delimited, do_parse, eof, line_ending, many0,
    many1, named, not, not_line_ending, pair, preceded, recognize, return_error, tag, tag_no_case,
    take_while1, terminated, value, ws, IResult,
};

use wayfinder_core::*;

#[macro_use]
pub mod errors;

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

indented!(
    path_segment<PathSegment>,
    alt_complete!(
        do_parse!(
            param:
                delimited!(
                    char!('{'),
                    ws!(param),
                    return_error!(err_str!("Expected a \"}\""), char!('}'))
                )
                >> (PathSegment::from(param))
        ) | do_parse!(text: take_while1!(is_identifier_char) >> (PathSegment::from(text)))
    )
);

indented!(
    query_parameter<Param>,
    delimited!(
        char!('['),
        ws!(param),
        return_error!(err_str!("Expected a \"]\""), char!(']'))
    )
);

named!(target<CompleteStr, (bool, Vec<String>, String)>,
    alt_complete!(
        do_parse!(
            ws!(tag!("->")) >>
            modules: many0!(terminated!(
                take_while1!(is_identifier_char),
                ws!(tag!("::"))
            )) >>
            action: take_while1!(is_identifier_char) >>
            ((true, modules.iter().map(|m| m.to_string()).collect(), action.to_string()))
        ) |
        do_parse!(
            modules: many0!(terminated!(
                take_while1!(is_identifier_char),
                ws!(tag!("::"))
            )) >>
            action: take_while1!(is_identifier_char) >>
            ((false, modules.iter().map(|m| m.to_string()).collect(), action.to_string()))
        )
    )
);

named!(require_newline<CompleteStr, ()>,
    return_error!(
        err_str!("Expected a newline"),
        value!((), many1!(line_ending))
    )
);

pub fn resource(input: CompleteStr, level: usize) -> IResult<CompleteStr, Resource> {
    preceded!(
        input,
        count!(tag!("  "), level),
        do_parse!(
            method: method
                >> char!(' ')
                >> target: target
                >> require_newline
                >> query_parameters:
                    many0!(terminated!(
                        apply!(query_parameter, level + 1),
                        require_newline
                    ))
                >> (Resource {
                    method,
                    modules: target.1,
                    name: target.2,
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
    return_error!(
        input,
        err_str!("Expected resources or child routes"),
        alt_complete!(
            do_parse!(
                resources: many1!(apply!(resource, level))
                    >> routes:
                        alt_complete!(
                            preceded!(many0!(line_ending), many1!(apply!(nested_routes, level)))
                                | value!(vec![])
                        )
                    >> ((resources, routes))
            ) | do_parse!(routes: many1!(apply!(nested_routes, level)) >> ((vec![], routes)))
        )
    )
}

pub fn routes(input: CompleteStr, level: usize) -> IResult<CompleteStr, Routes> {
    do_parse!(
        input,
        query_parameters:
            many0!(terminated!(
                apply!(query_parameter, level),
                many1!(line_ending)
            ))
            >> rnr: apply!(resources_and_routes, level)
            >> (Routes {
                resources: rnr.0,
                routes: rnr.1,
                query_parameters,
            })
    )
}

pub fn nested_routes(input: CompleteStr, level: usize) -> IResult<CompleteStr, NestedRoutes> {
    do_parse!(
        input,
        path_segment: apply!(path_segment, level)
            >> many1!(line_ending)
            >> routes:
                return_error!(
                    err_str!("In routes starting here"),
                    apply!(routes, level + 1)
                )
            >> (NestedRoutes {
                path_segment,
                routes
            })
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
    complete!(
        input,
        do_parse!(
            headers: many0!(header)
                >> many0!(line_ending)
                >> char!('/')
                >> many1!(line_ending)
                >> routes: apply!(routes, 1)
                >> eof!()
                >> ({
                    let mut headers = headers;
                    headers.retain(|h| h.text.len() != 0);
                    RouteConfig { headers, routes }
                })
        )
    )
}
