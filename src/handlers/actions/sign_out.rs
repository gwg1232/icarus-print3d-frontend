use axum::response::{IntoResponse, Redirect, Response};
use tower_sessions::Session;

use crate::{flash::FlashMessage, paths};

pub async fn post_actions_sign_out(
    session: Session,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    FlashMessage::info("You have been signed out.").set(&session).await?;
    session.flush().await?;
    Ok(Redirect::to(paths::pages::ROOT).into_response())
}
