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
        @get $modules:ident $name:ident $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Get,
            modules: vec![stringify!($modules).to_string()],
            name: stringify!($name).to_string(),
            is_redirect: $is_redirect,
            query_parameters: vec![$($param),*],
        }
    };
    (
        $modules:ident :: $name:ident $(, $param: expr)*
    ) => {
        get!(@get $modules $name false $(, $param)*)
    };
    (
        -> $modules:ident :: $name:ident
    ) => {
        get!(@get $modules $name true)
    };
}

#[macro_export]
macro_rules! post {
    (
        @post $modules:ident $name:ident $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Post,
            modules: vec![stringify!($modules).to_string()],
            name: stringify!($name).to_string(),
            is_redirect: $is_redirect,
            query_parameters: vec![$($param),*],
        }
    };
    (
        $modules:ident :: $name:ident $(, $param: expr)*
    ) => {
        post!(@post $modules $name false $(, $param)*)
    };
    (
        -> $modules:ident :: $name:ident
    ) => {
        post!(@post $modules $name true)
    };
}

#[macro_export]
macro_rules! put {
    (
        @put $modules:ident $name:ident $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Put,
            modules: vec![stringify!($modules).to_string()],
            name: stringify!($name).to_string(),
            is_redirect: $is_redirect,
            query_parameters: vec![$($param),*],
        }
    };
    (
        $modules:ident :: $name:ident $(, $param: expr)*
    ) => {
        put!(@put $modules $name false $(, $param)*)
    };
    (
        -> $modules:ident :: $name:ident
    ) => {
        put!(@put $modules $name true)
    };
}

#[macro_export]
macro_rules! delete {
    (
        @delete $modules:ident $name:ident $is_redirect:expr $(, $param:expr)*
    ) => {
        ::wayfinder_core::Resource {
            method: ::wayfinder_core::Method::Delete,
            modules: vec![stringify!($modules).to_string()],
            name: stringify!($name).to_string(),
            is_redirect: $is_redirect,
            query_parameters: vec![$($param),*],
        }
    };
    (
        $modules:ident :: $name:ident $(, $param: expr)*
    ) => {
        delete!(@delete $modules $name false $(, $param)*)
    };
    (
        -> $modules:ident :: $name:ident
    ) => {
        delete!(@delete $modules $name true)
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
