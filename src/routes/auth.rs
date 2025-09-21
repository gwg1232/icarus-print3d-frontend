use crate::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/sign_up",
            get(handlers::auth::sign_up_page).post(handlers::auth::sign_up),
        )
        .route(
            "/sign_in",
            get(handlers::auth::sign_in_page).post(handlers::auth::sign_in),
        )
        .route("/sign_out", post(handlers::auth::sign_out))
}
