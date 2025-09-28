use axum::{
    Form,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use validator::Validate;

use crate::models::dtos::user::{FIELD_EMAIL, FIELD_PASSWORD, SignInForm, SignUpForm};
use crate::views::pages::{sign_in, sign_up};

use super::helpers;

pub async fn post_forms_sign_up(Form(sign_up_form): Form<SignUpForm>) -> Response {
    match sign_up_form.validate() {
        Ok(_) => {
            tracing::info!("Sign up successful for email: {}", sign_up_form.email);
            Redirect::to("/").into_response()
        }
        Err(errs) => {
            let errors = helpers::parse_errors(&errs.to_string());

            (
                StatusCode::BAD_REQUEST,
                sign_up::sign_up(
                    errors.get(FIELD_EMAIL).map(|s| s.as_str()),
                    errors.get(FIELD_PASSWORD).map(|s| s.as_str()),
                ),
            )
                .into_response()
        }
    }
}

pub async fn post_forms_sign_in(Form(sign_in_form): Form<SignInForm>) -> Response {
    match sign_in_form.validate() {
        Ok(_) => Redirect::to("/").into_response(),
        Err(errs) => {
            let errors = helpers::parse_errors(&errs.to_string());

            (
                StatusCode::BAD_REQUEST,
                sign_in::sign_in(
                    errors.get(FIELD_EMAIL).map(|s| s.as_str()),
                    errors.get(FIELD_PASSWORD).map(|s| s.as_str()),
                ),
            )
                .into_response()
        }
    }
}

pub async fn post_forms_sign_out() -> Response {
    StatusCode::NOT_IMPLEMENTED.into_response()
}
