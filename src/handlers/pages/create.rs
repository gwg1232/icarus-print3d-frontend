use axum::Extension;
use crate::{auth::CurrentUser, flash::FlashMessage, handlers::errors::HandlerError, views::pages};
use maud::Markup;

pub async fn get_create(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    Ok(pages::create::create(&current_user, &flash))
}
