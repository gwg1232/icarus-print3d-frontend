use crate::{config::AppState, handlers::forms, paths::forms::relative};
use axum::{Router, routing::post};

pub fn public_form_routes() -> Router<AppState> {
    Router::new()
        .route(relative::SIGN_UP, post(forms::post_forms_sign_up))
        .route(relative::SIGN_IN, post(forms::post_forms_sign_in))
}

pub fn protected_form_routes() -> Router<AppState> {
    Router::new()
        .route(relative::CREATE_TODO, post(forms::post_forms_create_todo))
        .route(relative::TOGGLE_TODO, post(forms::post_forms_toggle_todo))
        .route(relative::DELETE_TODO, post(forms::post_forms_delete_todo))
        .route(
            "/print_order",
            post(forms::post_forms_print_order),
        )
}
