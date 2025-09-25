use axum::{
    Form,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};

use crate::models::dtos::user::{SignInUser, SignUpUser};

pub async fn post_forms_sign_up(Form(sign_up_user): Form<SignUpUser>) -> Response {
    tracing::info!(
        "Email is {} and the password is {}",
        sign_up_user.email,
        sign_up_user.password
    );
    Redirect::to("/").into_response()
}

pub async fn post_forms_sign_in(Form(sign_in_user): Form<SignInUser>) -> Response {
    tracing::info!(
        "Email is {} and the password is {}",
        sign_in_user.email,
        sign_in_user.password
    );
    Redirect::to("/").into_response()
}

pub async fn post_forms_sign_out() -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}
