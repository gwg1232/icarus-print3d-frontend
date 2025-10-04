mod http_tracing;

pub mod forms;
pub mod pages;

use axum::{middleware, Router};
use tower_http::services::ServeDir;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;

use crate::routes::http_tracing::create_http_trace_layer;
use crate::{config::AppState, middlewares, paths};

pub fn create_routes(
    state: AppState,
    session_layer: SessionManagerLayer<PostgresStore>,
) -> Router {
    Router::new()
        .merge(pages::routes())
        .nest(paths::forms::BASE, forms::routes())
        .with_state(state)
        .nest_service(paths::static_files::BASE, ServeDir::new("static"))
        .layer(middleware::from_fn(middlewares::authenticate))
        .layer(session_layer)
        .layer(create_http_trace_layer())
}
