use axum::{extract::State, http::StatusCode, response::{IntoResponse, Redirect, Response}, Form};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    data::queries,
    handlers::dtos::user::{FIELD_EMAIL, FIELD_PASSWORD, SignInForm},
    paths::pages,
    views::pages::sign_in,
};

use super::parse_validation_errors;

pub async fn post_forms_sign_in(
    State(db): State<PgPool>,
    Form(form): Form<SignInForm>,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    if let Err(validation_errors) = form.validate() {
        return Ok(render_validation_errors(&form, &validation_errors));
    }

    match queries::user::authenticate_user(&db, &form.email, &form.password).await {
        Ok(user_id) => {
            tracing::info!("Sign in successful for user_id: {}", user_id);
            Ok(Redirect::to(pages::ROOT).into_response())
        }
        Err(crate::data::errors::DataError::Unauthorized(msg)) => {
            Ok(render_auth_error(&form, msg))
        }
        Err(err) => Err(err.into()),
    }
}

fn render_validation_errors(
    form: &SignInForm,
    validation_errors: &validator::ValidationErrors,
) -> Response {
    let errors = parse_validation_errors(validation_errors);
    (
        StatusCode::BAD_REQUEST,
        sign_in::sign_in(
            Some(&form.email),
            errors.get(FIELD_EMAIL).map(String::as_str),
            errors.get(FIELD_PASSWORD).map(String::as_str),
        ),
    )
        .into_response()
}

fn render_auth_error(form: &SignInForm, message: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        sign_in::sign_in(Some(&form.email), Some(message), None),
    )
        .into_response()
}
