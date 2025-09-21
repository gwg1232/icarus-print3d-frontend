use crate::{handlers::forms, paths::forms::relative};
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new()
        .route(relative::SIGN_UP, post(forms::post_forms_sign_up))
        .route(relative::SIGN_IN, post(forms::post_forms_sign_in))
        .route(relative::SIGN_OUT, post(forms::post_forms_sign_out))
}
