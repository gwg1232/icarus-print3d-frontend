use crate::views;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::Markup;

pub async fn sign_up_page() -> Markup {
    views::pages::sign_up::sign_up()
}

pub async fn sign_in_page() -> Markup {
    views::pages::sign_in::sign_in()
}

pub async fn sign_up() -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

pub async fn sign_in() -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}

pub async fn sign_out() -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}
