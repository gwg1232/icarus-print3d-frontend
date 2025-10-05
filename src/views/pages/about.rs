use crate::{auth::CurrentUser, flash::FlashMessage, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn about(current_user: &CurrentUser, flash: &Option<FlashMessage>) -> Markup {
    let content = html! {
        h1 class="text-4xl font-bold text-gray-900" { "About" }
    };

    base_layout(current_user, flash, "About", "About page", content)
}
