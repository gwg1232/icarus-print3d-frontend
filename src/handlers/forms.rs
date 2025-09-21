use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn post_forms_sign_up() -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

pub async fn post_forms_sign_in() -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

pub async fn post_forms_sign_out() -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}