use axum::{
    Form,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use validator::Validate;

use crate::models::dtos::user::{SignInForm, SignUpForm};

pub async fn post_forms_sign_up(Form(sign_up_form): Form<SignUpForm>) -> Response {
    tracing::info!(
        "Email is {} and the password is {}",
        sign_up_form.email,
        sign_up_form.password
    );
    Redirect::to("/").into_response()
}

pub async fn post_forms_sign_in(Form(sign_in_form): Form<SignInForm>) -> Response {
    match sign_in_form.validate() {
        Ok(_) => Redirect::to("/").into_response(),
        Err(errs) => {
            let errs = errs.to_string();
            tracing::error!("Validation error: {:?}", errs);
            Redirect::to("/").into_response()
        }
    }
}

pub async fn post_forms_sign_out() -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}
