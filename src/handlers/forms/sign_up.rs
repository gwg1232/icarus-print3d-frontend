use axum::{Form, extract::State, http::StatusCode, response::{IntoResponse, Redirect, Response}};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    data::commands,
    handlers::dtos::user::{FIELD_EMAIL, FIELD_PASSWORD, SignUpForm},
    paths::pages,
    views::pages::sign_up,
};

use super::parse_validation_errors;

pub async fn post_forms_sign_up(
    State(db): State<PgPool>,
    Form(form): Form<SignUpForm>,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    if let Err(validation_errors) = form.validate() {
        return Ok(render_validation_errors(&form, &validation_errors));
    }

    match commands::user::create_user(&db, &form.email, &form.password).await {
        Ok(_) => {
            tracing::info!("Sign up successful for email: {}", form.email);
            Ok(Redirect::to(pages::SIGN_IN).into_response())
        }
        Err(crate::data::errors::DataError::Conflict(msg)) => {
            Ok(render_conflict_error(&form, msg))
        }
        Err(err) => Err(err.into()),
    }
}

fn render_validation_errors(
    form: &SignUpForm,
    validation_errors: &validator::ValidationErrors,
) -> Response {
    let errors = parse_validation_errors(validation_errors);
    (
        StatusCode::BAD_REQUEST,
        sign_up::sign_up(
            Some(&form.email),
            errors.get(FIELD_EMAIL).map(String::as_str),
            errors.get(FIELD_PASSWORD).map(String::as_str),
        ),
    )
        .into_response()
}

fn render_conflict_error(form: &SignUpForm, message: &str) -> Response {
    (
        StatusCode::CONFLICT,
        sign_up::sign_up(Some(&form.email), Some(message), None),
    )
        .into_response()
}
