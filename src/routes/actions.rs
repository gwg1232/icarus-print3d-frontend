use crate::{config::AppState, handlers::forms, paths::actions::relative};
use axum::{Router, routing::post};

pub fn action_routes() -> Router<AppState> {
    Router::new()
        .route(relative::SIGN_OUT, post(forms::post_forms_sign_out))
}
