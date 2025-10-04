use axum::Extension;
use crate::{auth::CurrentUser, views::pages};
use maud::Markup;

pub async fn get_root(Extension(current_user): Extension<CurrentUser>) -> Markup {
    pages::root::root(&current_user)
}

pub async fn get_about(Extension(current_user): Extension<CurrentUser>) -> Markup {
    pages::about::about(&current_user)
}

pub async fn get_create(Extension(current_user): Extension<CurrentUser>) -> Markup {
    pages::create::create(&current_user)
}

pub async fn get_sign_up(Extension(current_user): Extension<CurrentUser>) -> Markup {
    pages::sign_up::sign_up(&current_user, None, None, None)
}

pub async fn get_sign_in(Extension(current_user): Extension<CurrentUser>) -> Markup {
    pages::sign_in::sign_in(&current_user, None, None, None)
}
