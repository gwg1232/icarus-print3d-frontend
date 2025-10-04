use crate::{auth::CurrentUser, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn about(current_user: &CurrentUser) -> Markup {
    let content = html! {
        div class="container mx-auto px-4 py-8" {
            h1 class="text-3xl font-bold mb-4" {
                "About"
            }
        }
    };

    base_layout(current_user, "About", "About", content)
}
