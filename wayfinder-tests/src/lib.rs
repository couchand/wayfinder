#![allow(unused_imports)]
#![allow(dead_code)]

mod diff;
mod macros;

use nom::types::CompleteStr;

use wayfinder_core::*;
use wayfinder_parse as parse;

include!(concat!(env!("OUT_DIR"), "/cases.rs"));

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
                modules: vec!["person".to_string()],
                name: "show".to_string(),
                is_redirect: false,
                query_parameters: vec![],
            },
            "GET person::show\n",
        ),
        (
            Resource {
                method: Method::Get,
                modules: vec!["person".to_string()],
                name: "show".to_string(),
                is_redirect: true,
                query_parameters: vec![],
            },
            "GET -> person::show\n",
        ),
        (
            Resource {
                method: Method::Get,
                modules: vec!["person".to_string()],
                name: "show".to_string(),
                is_redirect: false,
                query_parameters: vec![Param::new("id", "usize")],
            },
            "GET person::show\n  [id: usize]\n",
        ),
        (
            Resource {
                method: Method::Get,
                modules: vec![],
                name: "index".to_string(),
                is_redirect: false,
                query_parameters: vec![],
            },
            "GET index\n",
        ),
        (
            Resource {
                method: Method::Get,
                modules: vec!["admin".to_string(), "person".to_string()],
                name: "show".to_string(),
                is_redirect: false,
                query_parameters: vec![],
            },
            "GET admin::person::show\n",
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
                        modules: vec!["People".to_string()],
                        name: "Index".to_string(),
                        is_redirect: false,
                        query_parameters: vec![],
                    },
                    Resource {
                        method: Method::Post,
                        modules: vec!["People".to_string()],
                        name: "New".to_string(),
                        is_redirect: false,
                        query_parameters: vec![],
                    },
                ],
                routes: vec![],
                query_parameters: vec![],
            },
            "GET People::Index\nPOST People::New\n",
        ),
        (
            Routes {
                resources: vec![Resource {
                    method: Method::Get,
                    modules: vec!["People".to_string()],
                    name: "Index".to_string(),
                    is_redirect: false,
                    query_parameters: vec![],
                }],
                routes: vec![],
                query_parameters: vec![Param::new("lang", "String")],
            },
            "[lang: String]\nGET People::Index\n",
        ),
        (
            Routes {
                resources: vec![Resource {
                    method: Method::Get,
                    modules: vec!["People".to_string()],
                    name: "Index".to_string(),
                    is_redirect: false,
                    query_parameters: vec![],
                }],
                routes: vec![NestedRoutes {
                    path_segment: PathSegment::from(Param::new("id", "Uuid")),
                    routes: Routes {
                        resources: vec![Resource {
                            method: Method::Get,
                            modules: vec!["People".to_string()],
                            name: "Show".to_string(),
                            is_redirect: false,
                            query_parameters: vec![],
                        }],
                        routes: vec![],
                        query_parameters: vec![],
                    },
                }],
                query_parameters: vec![],
            },
            "GET People::Index\n{id: Uuid}\n  GET People::Show\n",
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
                    resources: vec![Resource {
                        method: Method::Get,
                        modules: vec!["People".to_string()],
                        name: "Index".to_string(),
                        is_redirect: false,
                        query_parameters: vec![],
                    }],
                    routes: vec![],
                    query_parameters: vec![],
                },
            },
            "people\n  GET People::Index\n",
        ),
        (
            NestedRoutes {
                path_segment: PathSegment::from(Param::new("id", "Uuid")),
                routes: Routes {
                    resources: vec![
                        Resource {
                            method: Method::Get,
                            modules: vec!["People".to_string()],
                            name: "Show".to_string(),
                            is_redirect: false,
                            query_parameters: vec![],
                        },
                        Resource {
                            method: Method::Put,
                            modules: vec!["People".to_string()],
                            name: "Update".to_string(),
                            is_redirect: false,
                            query_parameters: vec![Param::new("name", "String")],
                        },
                    ],
                    routes: vec![],
                    query_parameters: vec![],
                },
            },
            "{id: Uuid}\n  GET People::Show\n  PUT People::Update\n    [name: String]\n",
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
    vec![
        (
            RouteConfig {
                headers: vec![Header {
                    text: "use uuid::Uuid;".to_string(),
                }],
                routes: Routes {
                    resources: vec![],
                    routes: vec![NestedRoutes {
                        path_segment: PathSegment::from("people"),
                        routes: Routes {
                            resources: vec![Resource {
                                method: Method::Get,
                                modules: vec!["People".to_string()],
                                name: "Index".to_string(),
                                is_redirect: false,
                                query_parameters: vec![],
                            }],
                            routes: vec![NestedRoutes {
                                path_segment: PathSegment::from(Param::new("id", "Uuid")),
                                routes: Routes {
                                    resources: vec![
                                        Resource {
                                            method: Method::Get,
                                            modules: vec!["People".to_string()],
                                            name: "Show".to_string(),
                                            is_redirect: false,
                                            query_parameters: vec![],
                                        },
                                        Resource {
                                            method: Method::Put,
                                            modules: vec!["People".to_string()],
                                            name: "Update".to_string(),
                                            is_redirect: false,
                                            query_parameters: vec![Param::new("name", "String")],
                                        },
                                    ],
                                    routes: vec![],
                                    query_parameters: vec![],
                                },
                            }],
                            query_parameters: vec![],
                        },
                    }],
                    query_parameters: vec![Param::new("lang", "String")],
                },
            },
            "use uuid::Uuid;

/
  [lang: String]
  people
    GET People::Index
    {id: Uuid}
      GET People::Show
      PUT People::Update
        [name: String]
",
        ),
        (
            RouteConfig {
                headers: vec![],
                routes: Routes {
                    resources: vec![],
                    routes: vec![
                        NestedRoutes {
                            path_segment: PathSegment::from("one"),
                            routes: Routes {
                                resources: vec![
                                    Resource {
                                        method: Method::Get,
                                        modules: vec!["One".to_string()],
                                        name: "Show".to_string(),
                                        is_redirect: false,
                                        query_parameters: vec![],
                                    },
                                    Resource {
                                        method: Method::Post,
                                        modules: vec!["One".to_string()],
                                        name: "Make".to_string(),
                                        is_redirect: false,
                                        query_parameters: vec![],
                                    },
                                ],
                                routes: vec![],
                                query_parameters: vec![],
                            },
                        },
                        NestedRoutes {
                            path_segment: PathSegment::from("two"),
                            routes: Routes {
                                resources: vec![
                                    Resource {
                                        method: Method::Get,
                                        modules: vec!["Two".to_string()],
                                        name: "Show".to_string(),
                                        is_redirect: false,
                                        query_parameters: vec![],
                                    },
                                    Resource {
                                        method: Method::Post,
                                        modules: vec!["Two".to_string()],
                                        name: "Make".to_string(),
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
            },
            "/
  one
    GET One::Show
    POST One::Make
  two
    GET Two::Show
    POST Two::Make
",
        ),
    ]
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
