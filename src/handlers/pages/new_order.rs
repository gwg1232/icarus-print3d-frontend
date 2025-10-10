use axum::Extension;
use maud::Markup;

use crate::{
    auth::CurrentUser,
    flash::FlashMessage,
    views::pages,
};

pub async fn get_new_order(
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Markup {
    current_user.require_authenticated();

    pages::print_orders::new_order_page(&current_user, &flash)
}
