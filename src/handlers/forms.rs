use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    data::commands,
    handlers::{
        dtos::user::{FIELD_EMAIL, FIELD_PASSWORD, SignInForm, SignUpForm},
        errors::HandlerError,
    },
    paths::pages,
    views::pages::{sign_in, sign_up},
};

pub async fn post_forms_sign_in(Form(form): Form<SignInForm>) -> Result<Response, HandlerError> {
    if let Err(validation_errors) = form.validate() {
        return Ok(render_sign_in_errors(&validation_errors));
    }

    Ok(Redirect::to("/").into_response())
}

pub async fn post_forms_sign_out() -> Result<Response, HandlerError> {
    Ok(StatusCode::NOT_IMPLEMENTED.into_response())
}

pub async fn post_forms_sign_up(
    State(db): State<PgPool>,
    Form(form): Form<SignUpForm>,
) -> Result<Response, HandlerError> {
    if let Err(validation_errors) = form.validate() {
        return Ok(render_sign_up_errors(&validation_errors));
    }

    commands::user::create_user(&db, &form.email, &form.password).await?;
    tracing::info!("Sign up successful for email: {}", form.email);
    Ok(Redirect::to(pages::SIGN_IN).into_response())
}

fn render_sign_in_errors(validation_errors: &validator::ValidationErrors) -> Response {
    let errors = parse_validation_errors(validation_errors);
    (
        StatusCode::BAD_REQUEST,
        sign_in::sign_in(
            errors.get(FIELD_EMAIL).map(String::as_str),
            errors.get(FIELD_PASSWORD).map(String::as_str),
        ),
    )
        .into_response()
}

fn render_sign_up_errors(validation_errors: &validator::ValidationErrors) -> Response {
    let errors = parse_validation_errors(validation_errors);
    (
        StatusCode::BAD_REQUEST,
        sign_up::sign_up(
            errors.get(FIELD_EMAIL).map(String::as_str),
            errors.get(FIELD_PASSWORD).map(String::as_str),
        ),
    )
        .into_response()
}

fn parse_validation_errors(
    validation_errors: &validator::ValidationErrors,
) -> std::collections::HashMap<String, String> {
    validation_errors
        .field_errors()
        .iter()
        .filter_map(|(field, errs)| {
            errs.first()
                .and_then(|e| e.message.as_ref())
                .map(|msg| (field.to_string(), msg.to_string()))
        })
        .collect()
}
