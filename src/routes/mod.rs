mod http_tracing;

pub mod auth;
pub mod pages;
pub mod todos;

use axum::Router;
use tower_http::services::ServeDir;

use crate::routes::http_tracing::create_http_trace_layer;

pub fn create_routes() -> Router {
    Router::new()
        .merge(pages::routes())
        .merge(auth::routes())
        // .merge(todos::routes())
        .nest_service("/static", ServeDir::new("static"))
        .layer(create_http_trace_layer())
}
