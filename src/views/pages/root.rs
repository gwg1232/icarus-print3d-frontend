use crate::{auth::CurrentUser, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn root(current_user: &CurrentUser) -> Markup {
    let content = html! {
        h1 class="text-5xl font-bold text-gray-900" { "Welcome" }
    };

    base_layout(current_user, "Home", "Home page", content)
}
