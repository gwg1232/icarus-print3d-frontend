use crate::handlers;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handlers::pages::home))
        .route("/about", get(handlers::pages::about))
        .route("/create", get(handlers::pages::create))
}
