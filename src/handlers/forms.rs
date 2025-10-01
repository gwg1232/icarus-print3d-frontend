use super::helpers;
use crate::{
    data::commands,
    handlers::dtos::user::{FIELD_EMAIL, FIELD_PASSWORD, SignInForm, SignUpForm},
    views::pages::{sign_in, sign_up},
};
use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use sqlx::PgPool;
use validator::Validate;

pub async fn post_forms_sign_up(
    State(db): State<PgPool>,
    Form(sign_up_form): Form<SignUpForm>,
) -> Response {
    match sign_up_form.validate() {
        Ok(_) => {
            match commands::user::create_user(&db, &sign_up_form.email, &sign_up_form.password)
                .await
            {
                Ok(_) => {
                    tracing::info!("Sign up successful for email: {}", sign_up_form.email);
                    Redirect::to("/").into_response()
                }
                Err(_) => {
                    // Handle database error
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        sign_up::sign_up(None, Some("Failed to create account")),
                    )
                        .into_response()
                }
            }
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
