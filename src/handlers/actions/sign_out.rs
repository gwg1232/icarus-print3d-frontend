use axum::response::{IntoResponse, Redirect, Response};
use tower_sessions::Session;

use crate::{flash::FlashMessage, paths};

pub async fn post_actions_sign_out(
    session: Session,
) -> Result<Response, crate::handlers::errors::HandlerError> {
    session.flush().await?;
    FlashMessage::info("You have been signed out.").set(&session).await?;
    Ok(Redirect::to(paths::pages::ROOT).into_response())
}
