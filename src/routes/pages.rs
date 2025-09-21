use crate::handlers::pages;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(pages::get_root))
        .route("/about", get(pages::get_about))
        .route("/create", get(pages::get_create))
        .route("/sign_up", get(pages::get_sign_up))
        .route("/sign_in", get(pages::get_sign_in))
}
