#[macro_export]
macro_rules! header {
    (
        $($tokens:item)*
    ) => {
        ::wayfinder_core::Header::new(stringify!($($tokens)*))
    }
}

#[macro_export]
macro_rules! get {
    (
        @get $controller:ident $action:ident $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Get,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: $is_redirect,
            query_parameters: vec![$($param),*],
        }
    };
    (
        $controller:ident :: $action:ident $(, $param: expr)*
    ) => {
        get!(@get $controller $action false $(, $param)*)
    };
    (
        -> $controller:ident :: $action:ident
    ) => {
        get!(@get $controller $action true)
    };
}

#[macro_export]
macro_rules! post {
    (
        @post $controller:ident $action:ident $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Post,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: $is_redirect,
            query_parameters: vec![$($param),*],
        }
    };
    (
        $controller:ident :: $action:ident $(, $param: expr)*
    ) => {
        post!(@post $controller $action false $(, $param)*)
    };
    (
        -> $controller:ident :: $action:ident
    ) => {
        post!(@post $controller $action true)
    };
}

#[macro_export]
macro_rules! put {
    (
        @put $controller:ident $action:ident $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Put,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: $is_redirect,
            query_parameters: vec![$($param),*],
        }
    };
    (
        $controller:ident :: $action:ident $(, $param: expr)*
    ) => {
        put!(@put $controller $action false $(, $param)*)
    };
    (
        -> $controller:ident :: $action:ident
    ) => {
        put!(@put $controller $action true)
    };
}

#[macro_export]
macro_rules! delete {
    (
        @delete $controller:ident $action:ident $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Delete,
            controller: stringify!($controller).to_string(),
            action: stringify!($action).to_string(),
            is_redirect: $is_redirect,
            query_parameters: vec![$($param),*],
        }
    };
    (
        $controller:ident :: $action:ident $(, $param: expr)*
    ) => {
        delete!(@delete $controller $action false $(, $param)*)
    };
    (
        -> $controller:ident :: $action:ident
    ) => {
        delete!(@delete $controller $action true)
    };
}

#[macro_export]
macro_rules! param {
    (
        $name:ident : $type:ty
    ) => {
        ::wayfinder_core::Param::new(stringify!($name), stringify!($type))
    };
}
