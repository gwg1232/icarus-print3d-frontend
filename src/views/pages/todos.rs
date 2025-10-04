use crate::{auth::CurrentUser, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn todos(current_user: &CurrentUser) -> Markup {
    let content = html! {
        h1 class="text-4xl font-bold text-gray-900" { "Todos" }
    };

    base_layout(current_user, "Todos", "Todos page", content)
}