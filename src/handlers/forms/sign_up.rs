use axum::{Extension, Form, extract::State, http::StatusCode, response::{IntoResponse, Redirect, Response}};
use sqlx::PgPool;
use tower_sessions::Session;
use validator::Validate;

use crate::{
    auth::CurrentUser,
    data::commands,
    flash::FlashMessage,
    handlers::dtos::user::{FIELD_EMAIL, FIELD_PASSWORD, SignUpForm},
    paths::pages,
    views::pages::sign_up,
};

use super::parse_validation_errors;

pub async fn post_forms_sign_up(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Form(form): Form<SignUpForm>,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    if let Err(validation_errors) = form.validate() {
        return Ok(render_validation_errors(&current_user, &form, &validation_errors));
    }

    match commands::user::create_user(&db, &form.email, &form.password).await {
        Ok(_) => {
            FlashMessage::success("Account created successfully! Please sign in.").set(&session).await?;
            Ok(Redirect::to(pages::SIGN_IN).into_response())
        }
        Err(crate::data::errors::DataError::Conflict(msg)) => {
            FlashMessage::error(msg).set(&session).await?;
            Ok(Redirect::to(pages::SIGN_UP).into_response())
        }
        Err(err) => Err(err.into()),
    }
}

fn render_validation_errors(
    current_user: &CurrentUser,
    form: &SignUpForm,
    validation_errors: &validator::ValidationErrors,
) -> Response {
    let errors = parse_validation_errors(validation_errors);
    (
        StatusCode::BAD_REQUEST,
        sign_up::sign_up(
            current_user,
            &None,
            Some(&form.email),
            errors.get(FIELD_EMAIL).map(String::as_str),
            errors.get(FIELD_PASSWORD).map(String::as_str),
        ),
    )
        .into_response()
}
