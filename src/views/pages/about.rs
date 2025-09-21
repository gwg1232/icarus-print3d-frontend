use crate::views::layout::base::base_layout;
use maud::{Markup, html};

pub fn about() -> Markup {
    let content = html! {
        div class="container mx-auto px-4 py-8" {
            h1 class="text-3xl font-bold mb-4" {
                "About"
            }
        }
    };

    base_layout("About", "About", content)
}
