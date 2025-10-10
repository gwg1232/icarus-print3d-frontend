use axum::{Extension, extract::State};
use maud::Markup;
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    data::queries,
    flash::FlashMessage,
    handlers::errors::HandlerError,
    views::pages,
};

pub async fn get_print_orders(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated();

    let orders = queries::print_order::find_by_user_id(&db, user_id).await?;

    Ok(pages::print_orders::orders_page(&current_user, &flash, orders))
}
