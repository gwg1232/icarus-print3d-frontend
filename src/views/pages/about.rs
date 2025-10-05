use crate::{auth::CurrentUser, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn about(current_user: &CurrentUser) -> Markup {
    let content = html! {
        h1 class="text-4xl font-bold text-gray-900" { "About" }
    };

    base_layout(current_user, "About", "About page", content)
}
