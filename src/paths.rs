/// Macro that creates URL path constants with a base prefix.
///
/// # What it does
/// Takes a base path like "/forms" and creates constants for each route.
/// Each route gets TWO versions: relative and absolute.
///
/// # Simple Example
/// ```
/// define_nested_routes!("/api", {
///     USERS => "/users",
/// });
/// ```
///
/// This creates:
/// - `BASE` = "/api"
/// - `relative::USERS` = "/users"
/// - `USERS` = "/api/users" (BASE + relative::USERS)
///
/// # Usage
/// ```
/// // Register route handler
/// app.route(forms::SIGN_IN, post(handler))  // Uses "/forms/sign_in"
///
/// // Access just the path segment
/// let path = forms::relative::SIGN_IN;  // Just "/sign_in"
///
/// // Access the base
/// let base = forms::BASE;  // Just "/forms"
/// ```
macro_rules! define_nested_routes {
    ($base:expr, { $($name:ident => $path:expr),* $(,)? }) => {
        pub const BASE: &str = $base;

        #[allow(dead_code)]
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

pub mod print_orders {
    pub const BASE: &str = "/print_orders";

    pub mod new_order {
        pub const PATH: &str = "/print_orders/new";
    }
}

pub mod forms {
    define_nested_routes!("/forms", {
        SIGN_UP => "/sign_up",
        SIGN_IN => "/sign_in",
        CREATE_TODO => "/create_todo",
        TOGGLE_TODO => "/toggle_todo",
        DELETE_TODO => "/delete_todo",
    });

    pub mod print_order {
        pub const PATH: &str = "/forms/print_order";
    }
}

pub mod actions {
    define_nested_routes!("/actions", {
        SIGN_OUT => "/sign_out",
    });
}

pub mod static_files {
    define_nested_routes!("/static", {
        FAVICON => "/img/favicon.svg",
    });
}
