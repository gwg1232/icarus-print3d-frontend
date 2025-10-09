use axum::Extension;
use crate::{auth::CurrentUser, flash::FlashMessage, handlers::errors::HandlerError, views::pages};
use maud::Markup;

pub async fn get_sign_up(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    Ok(pages::sign_up::sign_up(&current_user, &flash, None, None, None))
}
