use crate::views::layout::base::base_layout;
use maud::{Markup, html};

pub fn todos() -> Markup {
    let content = html! {
        h1 class="text-3xl font-bold mb-4" {
            "This is todos page"
        }
    };

    base_layout("Todos", "Todos", content)
}