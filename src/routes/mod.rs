mod http_tracing;

mod actions;
mod forms;
mod pages;

use axum::{Router, middleware};
use tower_http::services::ServeDir;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;

use crate::{config::AppState, handlers, middlewares, paths};
use http_tracing::create_http_trace_layer;

pub fn create_routes(state: AppState, session_layer: SessionManagerLayer<PostgresStore>) -> Router {
    let protected_routes =
        pages::protected_routes().layer(middleware::from_fn(middlewares::require_authentication));

    let app_routes = Router::new()
        .merge(pages::public_routes())
        .merge(protected_routes)
        .nest(paths::forms::BASE, forms::form_routes())
        .nest(paths::actions::BASE, actions::action_routes())
        .fallback(handlers::fallback::handle_404)
        .with_state(state)
        .layer(middleware::from_fn(middlewares::authenticate))
        .layer(session_layer);

    Router::new()
        .nest_service(paths::static_files::BASE, ServeDir::new("static"))
        .merge(app_routes)
        .layer(create_http_trace_layer())
}
