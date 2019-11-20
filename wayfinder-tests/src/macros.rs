use wayfinder_core::{delete, get, header, param, post, put, Header, Method, Param, Resource};

#[test]
fn test_header_macro() {
    let h = header!(use uuid::Uuid;);

    assert_eq!(h, Header::new("use uuid::Uuid;"));
}

#[test]
fn test_param_macro() {
    let p = param!(id: Uuid);

    assert_eq!(p, Param::new("id", "Uuid"));
}

#[test]
fn test_get_macro_basic() {
    let g = get!(User::New);

    assert_eq!(g, Resource {
        method: Method::Get,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: false,
        query_parameters: vec![],
    });
}

#[test]
fn test_get_macro_redirect() {
    let g = get!(-> User::New);

    assert_eq!(g, Resource {
        method: Method::Get,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: true,
        query_parameters: vec![],
    });
}

#[test]
fn test_get_macro_params() {
    let g = get!(User::New, param!(name: String));

    assert_eq!(g, Resource {
        method: Method::Get,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: false,
        query_parameters: vec![param!(name: String)],
    });
}

#[test]
fn test_post_macro_basic() {
    let g = post!(User::New);

    assert_eq!(g, Resource {
        method: Method::Post,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: false,
        query_parameters: vec![],
    });
}

#[test]
fn test_post_macro_redirect() {
    let g = post!(-> User::New);

    assert_eq!(g, Resource {
        method: Method::Post,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: true,
        query_parameters: vec![],
    });
}

#[test]
fn test_post_macro_params() {
    let g = post!(User::New, param!(name: String));

    assert_eq!(g, Resource {
        method: Method::Post,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: false,
        query_parameters: vec![param!(name: String)],
    });
}

#[test]
fn test_put_macro_basic() {
    let g = put!(User::New);

    assert_eq!(g, Resource {
        method: Method::Put,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: false,
        query_parameters: vec![],
    });
}

#[test]
fn test_put_macro_redirect() {
    let g = put!(-> User::New);

    assert_eq!(g, Resource {
        method: Method::Put,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: true,
        query_parameters: vec![],
    });
}

#[test]
fn test_put_macro_params() {
    let g = put!(User::New, param!(name: String));

    assert_eq!(g, Resource {
        method: Method::Put,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: false,
        query_parameters: vec![param!(name: String)],
    });
}

#[test]
fn test_delete_macro_basic() {
    let g = delete!(User::New);

    assert_eq!(g, Resource {
        method: Method::Delete,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: false,
        query_parameters: vec![],
    });
}

#[test]
fn test_delete_macro_redirect() {
    let g = delete!(-> User::New);

    assert_eq!(g, Resource {
        method: Method::Delete,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: true,
        query_parameters: vec![],
    });
}

#[test]
fn test_delete_macro_params() {
    let g = delete!(User::New, param!(name: String));

    assert_eq!(g, Resource {
        method: Method::Delete,
        modules: vec!["User".into()],
        name: "New".into(),
        is_redirect: false,
        query_parameters: vec![param!(name: String)],
    });
}
