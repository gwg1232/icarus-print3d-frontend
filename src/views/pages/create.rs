use crate::{auth::CurrentUser, flash::FlashMessage, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn create(current_user: &CurrentUser, flash: &Option<FlashMessage>) -> Markup {
    let content = html! {
        h1 class="text-4xl font-bold text-gray-900" { "Create" }
    };

    base_layout(current_user, flash, "Create", "Create page", content)
}