use crate::{auth::CurrentUser, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn create(current_user: &CurrentUser) -> Markup {
    let content = html! {
        h1 class="text-3xl font-bold mb-4" {
            "This is create page"
        }
    };

    base_layout(current_user, "Create", "Create", content)
}