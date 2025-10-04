use crate::{auth::CurrentUser, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn not_found(current_user: &CurrentUser) -> Markup {
    let content = html! {
        h1 class="text-6xl font-bold text-gray-900" { "404" }
    };

    base_layout(current_user, "Page Not Found", "Page not found", content)
}
