mod http_tracing;

pub mod forms;
pub mod pages;

use axum::Router;
use tower_http::services::ServeDir;

use crate::routes::http_tracing::create_http_trace_layer;
use crate::{config::AppState, paths};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .merge(pages::routes())
        .nest(paths::forms::BASE, forms::routes())
        .with_state(state)
        .nest_service(paths::static_files::BASE, ServeDir::new("static"))
        .layer(create_http_trace_layer())
}
