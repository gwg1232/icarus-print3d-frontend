use axum::{Extension, extract::State};
use crate::{auth::CurrentUser, data::queries, flash::FlashMessage, views::pages};
use maud::Markup;
use sqlx::PgPool;

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
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Markup {
    let CurrentUser::Authenticated { user_id } = current_user else {
        unreachable!("Protected route accessed by guest user");
    };

    let todos = queries::todo::get_todos_for_user(&db, user_id)
        .await
        .unwrap_or_else(|_| vec![]);

    pages::todos::todos(&current_user, &flash, todos, None, None)
}

