use axum::{Extension, http::StatusCode};
use maud::Markup;

use crate::{auth::CurrentUser, views::pages};

pub(crate) async fn handle_404(
    Extension(current_user): Extension<CurrentUser>,
) -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, pages::not_found::not_found(&current_user))
}
