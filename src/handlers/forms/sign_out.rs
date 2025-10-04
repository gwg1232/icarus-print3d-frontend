use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub async fn post_forms_sign_out() -> Result<Response, crate::handlers::errors::HandlerError> {
    Ok(StatusCode::NOT_IMPLEMENTED.into_response())
}
