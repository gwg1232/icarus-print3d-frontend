macro_rules! define_nested_routes {
    ($base:expr, { $($name:ident => $path:expr),* $(,)? }) => {
        pub const BASE: &str = $base;

        pub mod relative {
            $(pub const $name: &str = $path;)*
        }

        $(pub const $name: &str = concat!($base, $path);)*
    };
}

pub mod pages {
    pub const ROOT: &str = "/";
    pub const SIGN_UP: &str = "/sign_up";
    pub const SIGN_IN: &str = "/sign_in";
    pub const ABOUT: &str = "/about";
    pub const CREATE: &str = "/create";
    pub const TODOS: &str = "/todos";
}

pub mod forms {
    define_nested_routes!("/forms", {
        SIGN_UP => "/sign_up",
        SIGN_IN => "/sign_in",
        SIGN_OUT => "/sign_out",
    });
}

pub mod static_files {
    define_nested_routes!("/static", {
        FAVICON => "/img/favicon.svg",
    });
}
