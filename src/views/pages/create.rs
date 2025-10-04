use crate::{auth::CurrentUser, views::layout::base::base_layout};
use maud::{Markup, html};

pub(crate) fn create(current_user: &CurrentUser) -> Markup {
    let content = html! {
        h1 class="text-4xl font-bold text-gray-900" { "Create" }
    };

    base_layout(current_user, "Create", "Create page", content)
}