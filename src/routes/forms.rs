use crate::{config::AppState, handlers::forms, paths::forms::relative};
use axum::{Router, routing::post};

pub(crate) fn form_routes() -> Router<AppState> {
    Router::new()
        .route(relative::SIGN_UP, post(forms::post_forms_sign_up))
        .route(relative::SIGN_IN, post(forms::post_forms_sign_in))
}
