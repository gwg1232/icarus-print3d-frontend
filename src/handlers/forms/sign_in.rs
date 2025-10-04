use axum::{
    Extension,
    Form,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use sqlx::PgPool;
use tower_sessions::Session;
use validator::Validate;

use crate::{
    auth::{CurrentUser, SESSION_USER_ID_KEY},
    data::queries,
    handlers::dtos::user::{FIELD_EMAIL, FIELD_PASSWORD, SignInForm},
    paths::pages,
    views::pages::sign_in,
};

use super::parse_validation_errors;

pub(crate) async fn post_forms_sign_in(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Form(form): Form<SignInForm>,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    if let Err(validation_errors) = form.validate() {
        return Ok(render_validation_errors(&current_user, &form, &validation_errors));
    }

    match queries::user::authenticate_user(&db, &form.email, &form.password).await {
        Ok(user_id) => {
            tracing::info!("Sign in successful for user_id: {}", user_id);
            session.insert(SESSION_USER_ID_KEY, user_id).await?;
            Ok(Redirect::to(pages::ROOT).into_response())
        }
        Err(crate::data::errors::DataError::Unauthorized(msg)) => Ok(render_auth_error(&current_user, &form, msg)),
        Err(err) => Err(err.into()),
    }
}

fn render_validation_errors(
    current_user: &CurrentUser,
    form: &SignInForm,
    validation_errors: &validator::ValidationErrors,
) -> Response {
    let errors = parse_validation_errors(validation_errors);
    (
        StatusCode::BAD_REQUEST,
        sign_in::sign_in(
            current_user,
            Some(&form.email),
            errors.get(FIELD_EMAIL).map(String::as_str),
            errors.get(FIELD_PASSWORD).map(String::as_str),
        ),
    )
        .into_response()
}

fn render_auth_error(current_user: &CurrentUser, form: &SignInForm, message: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        sign_in::sign_in(current_user, Some(&form.email), Some(message), None),
    )
        .into_response()
}
