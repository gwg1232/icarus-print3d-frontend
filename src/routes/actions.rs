use crate::{config::AppState, handlers::actions, paths::actions::relative};
use axum::{Router, routing::post};

pub(crate) fn action_routes() -> Router<AppState> {
    Router::new()
        .route(relative::SIGN_OUT, post(actions::post_actions_sign_out))
}
