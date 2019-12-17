#[macro_export]
macro_rules! header {
    (
        $($tokens:item)*
    ) => {
        ::wayfinder::Header::new(stringify!($($tokens)*))
    }
}

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

#[macro_export]
macro_rules! param {
    (
        $name:ident : $type:ty
    ) => {
        ::wayfinder::Param::new(stringify!($name), stringify!($type))
    };
}
