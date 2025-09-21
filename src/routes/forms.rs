use crate::handlers::forms;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new()
        .route("/sign_up", post(forms::post_forms_sign_up))
        .route("/sign_in", post(forms::post_forms_sign_in))
        .route("/sign_out", post(forms::post_forms_sign_out))
}
