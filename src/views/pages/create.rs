use crate::views::layout::base::base_layout;
use maud::{Markup, html};

pub fn create() -> Markup {
    let content = html! {
        h1 class="text-3xl font-bold mb-4" {
            "This is create page"
        }
    };

    base_layout("Create", "Create", content)
}