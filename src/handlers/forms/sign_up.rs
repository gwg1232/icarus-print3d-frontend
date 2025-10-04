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
        return Ok(render_validation_errors(&validation_errors));
    }

    match commands::user::create_user(&db, &form.email, &form.password).await {
        Ok(_) => {
            tracing::info!("Sign up successful for email: {}", form.email);
            Ok(Redirect::to(pages::SIGN_IN).into_response())
        }
        Err(err) => {
            if is_duplicate_email_error(&err) {
                Ok(render_database_error(&err))
            } else {
                Err(err.into())
            }
        }
    }
}

fn render_validation_errors(validation_errors: &validator::ValidationErrors) -> Response {
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

fn is_duplicate_email_error(err: &crate::data::errors::DataError) -> bool {
    matches!(err, crate::data::errors::DataError::DuplicateEmail)
}

fn render_database_error(err: &crate::data::errors::DataError) -> Response {
    (
        StatusCode::BAD_REQUEST,
        sign_up::sign_up(Some(&err.to_string()), None),
    )
        .into_response()
}
