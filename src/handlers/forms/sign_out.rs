use axum::response::{IntoResponse, Redirect, Response};
use tower_sessions::Session;

use crate::paths;

pub async fn post_forms_sign_out(
    session: Session,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    session.flush().await?;
    Ok(Redirect::to(paths::pages::ROOT).into_response())
}
