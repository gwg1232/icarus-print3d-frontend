use axum::Extension;
use crate::{auth::CurrentUser, flash::FlashMessage, views::pages};
use maud::Markup;

pub async fn get_root(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Markup {
    pages::root::root(&current_user, &flash)
}

pub async fn get_about(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Markup {
    pages::about::about(&current_user, &flash)
}

pub async fn get_create(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Markup {
    pages::create::create(&current_user, &flash)
}

pub async fn get_sign_up(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Markup {
    pages::sign_up::sign_up(&current_user, &flash, None, None, None)
}

pub async fn get_sign_in(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Markup {
    pages::sign_in::sign_in(&current_user, &flash, None, None, None)
}

pub async fn get_todos(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Markup {
    pages::todos::todos(&current_user, &flash)
}

