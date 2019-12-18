/// Create a [`Header`](struct.Header.html), for instance to `use` a data type.
///
/// ```
/// # use wayfinder::{header, Header};
/// let import_uuid = header!(
///     use uuid::Uuid;
/// );
/// # assert_eq!(import_uuid, Header::new("use uuid::Uuid;"));
/// ```
#[macro_export]
macro_rules! header {
    (
        $($tokens:item)*
    ) => {
        ::wayfinder::Header::new(stringify!($($tokens)*))
    }
}

/// Create a [`Resource`](struct.Resource.html) for an HTTP GET request.
///
/// ```
/// # use wayfinder::{get, Resource, Method};
/// let show_person = get!(People::Show);
/// # assert_eq!(show_person, Resource {
/// #     method: Method::Get,
/// #     modules: vec!["People".to_string()],
/// #     name: "Show".to_string(),
/// #     is_redirect: false,
/// #     query_parameters: vec![],
/// # });
/// ```
///
/// Use the sigil `->` at the start to indicate a redirect-style route.
///
/// ```
/// # use wayfinder::{get, Resource, Method};
/// let show_person_redirect = get!(-> People::Show);
/// # assert_eq!(show_person_redirect, Resource {
/// #     method: Method::Get,
/// #     modules: vec!["People".to_string()],
/// #     name: "Show".to_string(),
/// #     is_redirect: true,
/// #     query_parameters: vec![],
/// # });
/// ```
///
/// Add query parameters after the handler name.
///
/// ```
/// # use wayfinder::{get, param, Resource, Method, Param};
/// let search = get!(Search, param!(q: String));
/// # assert_eq!(search, Resource {
/// #     method: Method::Get,
/// #     modules: vec![],
/// #     name: "Search".to_string(),
/// #     is_redirect: false,
/// #     query_parameters: vec![param!(q: String)],
/// # });
/// ```
#[macro_export]
macro_rules! get {
    (
        @get ($($modules:ident)+) $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder::Resource::make(
            ::wayfinder::Method::Get,
            vec![$(stringify!($modules).to_string()),*],
            $is_redirect,
            vec![$($param),*],
        )
    };
    (
        $root:ident $(:: $nested:ident)* $(, $param: expr)*
    ) => {
        get!(@get ($root $($nested)*) false $(, $param)*)
    };
    (
        -> $root:ident $(:: $nested:ident)*
    ) => {
        get!(@get ($root $($nested)*) true)
    };
}

/// Create a [`Resource`](struct.Resource.html) for an HTTP POST request.
///
/// ```
/// # use wayfinder::{post, Resource, Method};
/// let create_person = post!(People::Create);
/// # assert_eq!(create_person, Resource {
/// #     method: Method::Post,
/// #     modules: vec!["People".to_string()],
/// #     name: "Create".to_string(),
/// #     is_redirect: false,
/// #     query_parameters: vec![],
/// # });
/// ```
///
/// Use the sigil `->` at the start to indicate a redirect-style route.
///
/// ```
/// # use wayfinder::{post, Resource, Method};
/// let create_person_redirect = post!(-> People::Create);
/// # assert_eq!(create_person_redirect, Resource {
/// #     method: Method::Post,
/// #     modules: vec!["People".to_string()],
/// #     name: "Create".to_string(),
/// #     is_redirect: true,
/// #     query_parameters: vec![],
/// # });
/// ```
///
/// Add query parameters after the handler name.
///
/// ```
/// # use wayfinder::{post, param, Resource, Method, Param};
/// let search = post!(Search, param!(q: String));
/// # assert_eq!(search, Resource {
/// #     method: Method::Post,
/// #     modules: vec![],
/// #     name: "Search".to_string(),
/// #     is_redirect: false,
/// #     query_parameters: vec![param!(q: String)],
/// # });
/// ```
#[macro_export]
macro_rules! post {
    (
        @post ($($modules:ident)+) $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder::Resource::make(
            ::wayfinder::Method::Post,
            vec![$(stringify!($modules).to_string()),*],
            $is_redirect,
            vec![$($param),*],
        )
    };
    (
        $root:ident $(:: $nested:ident)* $(, $param: expr)*
    ) => {
        post!(@post ($root $($nested)*) false $(, $param)*)
    };
    (
        -> $root:ident $(:: $nested:ident)*
    ) => {
        post!(@post ($root $($nested)*) true)
    };
}

/// Create a [`Resource`](struct.Resource.html) for an HTTP PUT request.
///
/// ```
/// # use wayfinder::{put, Resource, Method};
/// let update_person = put!(People::Update);
/// # assert_eq!(update_person, Resource {
/// #     method: Method::Put,
/// #     modules: vec!["People".to_string()],
/// #     name: "Update".to_string(),
/// #     is_redirect: false,
/// #     query_parameters: vec![],
/// # });
/// ```
///
/// Use the sigil `->` at the start to indicate a redirect-style route.
///
/// ```
/// # use wayfinder::{put, Resource, Method};
/// let update_person_redirect = put!(-> People::Update);
/// # assert_eq!(update_person_redirect, Resource {
/// #     method: Method::Put,
/// #     modules: vec!["People".to_string()],
/// #     name: "Update".to_string(),
/// #     is_redirect: true,
/// #     query_parameters: vec![],
/// # });
/// ```
///
/// Add query parameters after the handler name.
///
/// ```
/// # use wayfinder::{put, param, Resource, Method, Param};
/// let update_person_by_id = put!(Person::Update, param!(id: i32));
/// # assert_eq!(update_person_by_id, Resource {
/// #     method: Method::Put,
/// #     modules: vec!["Person".to_string()],
/// #     name: "Update".to_string(),
/// #     is_redirect: false,
/// #     query_parameters: vec![param!(id: i32)],
/// # });
/// ```
#[macro_export]
macro_rules! put {
    (
        @put ($($modules:ident)+) $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder::Resource::make(
            ::wayfinder::Method::Put,
            vec![$(stringify!($modules).to_string()),*],
            $is_redirect,
            vec![$($param),*],
        )
    };
    (
        $root:ident $(:: $nested:ident)* $(, $param: expr)*
    ) => {
        put!(@put ($root $($nested)*) false $(, $param)*)
    };
    (
        -> $root:ident $(:: $nested:ident)*
    ) => {
        put!(@put ($root $($nested)*) true)
    };
}

/// Create a [`Resource`](struct.Resource.html) for an HTTP DELETE request.
///
/// ```
/// # use wayfinder::{delete, Resource, Method};
/// let destroy_person = delete!(People::Destroy);
/// # assert_eq!(destroy_person, Resource {
/// #     method: Method::Delete,
/// #     modules: vec!["People".to_string()],
/// #     name: "Destroy".to_string(),
/// #     is_redirect: false,
/// #     query_parameters: vec![],
/// # });
/// ```
///
/// Use the sigil `->` at the start to indicate a redirect-style route.
///
/// ```
/// # use wayfinder::{delete, Resource, Method};
/// let destroy_person_redirect = delete!(-> People::Destroy);
/// # assert_eq!(destroy_person_redirect, Resource {
/// #     method: Method::Delete,
/// #     modules: vec!["People".to_string()],
/// #     name: "Destroy".to_string(),
/// #     is_redirect: true,
/// #     query_parameters: vec![],
/// # });
/// ```
///
/// Add query parameters after the handler name.
///
/// ```
/// # use wayfinder::{delete, param, Resource, Method, Param};
/// let destroy_person_by_id = delete!(Person::Destroy, param!(id: i32));
/// # assert_eq!(destroy_person_by_id, Resource {
/// #     method: Method::Delete,
/// #     modules: vec!["Person".to_string()],
/// #     name: "Destroy".to_string(),
/// #     is_redirect: false,
/// #     query_parameters: vec![param!(id: i32)],
/// # });
/// ```
#[macro_export]
macro_rules! delete {
    (
        @delete ($($modules:ident)+) $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder::Resource::make(
            ::wayfinder::Method::Delete,
            vec![$(stringify!($modules).to_string()),*],
            $is_redirect,
            vec![$($param),*],
        )
    };
    (
        $root:ident $(:: $nested:ident)* $(, $param: expr)*
    ) => {
        delete!(@delete ($root $($nested)*) false $(, $param)*)
    };
    (
        -> $root:ident $(:: $nested:ident)*
    ) => {
        delete!(@delete ($root $($nested)*) true)
    };
}

/// Create a [`Param`](struct.Param.html) to use as a path segment or
/// query parameter.
///
/// ```
/// # use wayfinder::{param, Param};
/// let username = param!(username: String);
/// # assert_eq!(username, Param::new("username", "String"));
/// ```
#[macro_export]
macro_rules! param {
    (
        $name:ident : $type:ty
    ) => {
        ::wayfinder::Param::new(stringify!($name), stringify!($type))
    };
}
