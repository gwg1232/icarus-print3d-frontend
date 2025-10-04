use axum::{Form, http::StatusCode, response::{IntoResponse, Redirect, Response}};
use validator::Validate;

use crate::{
    handlers::dtos::user::{FIELD_EMAIL, FIELD_PASSWORD, SignInForm},
    views::pages::sign_in,
};

use super::parse_validation_errors;

pub async fn post_forms_sign_in(
    Form(form): Form<SignInForm>,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    if let Err(validation_errors) = form.validate() {
        return Ok(render_errors(&validation_errors));
    }

    Ok(Redirect::to("/").into_response())
}

fn render_errors(validation_errors: &validator::ValidationErrors) -> Response {
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
