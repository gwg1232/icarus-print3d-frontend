use axum::Extension;
use crate::{auth::CurrentUser, flash::FlashMessage, handlers::errors::HandlerError, views::pages};
use maud::Markup;

pub async fn get_about(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    Ok(pages::about::about(&current_user, &flash))
}
