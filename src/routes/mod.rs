mod http_tracing;

pub mod forms;
pub mod pages;

use axum::Router;
use tower_http::services::ServeDir;

use crate::paths;
use crate::routes::http_tracing::create_http_trace_layer;

pub fn create_routes() -> Router {
    Router::new()
        .merge(pages::routes())
        .nest(paths::forms::BASE, forms::routes())
        .nest_service(paths::static_files::BASE, ServeDir::new("static"))
        .layer(create_http_trace_layer())
}
