use crate::views::layout::base::base_layout;
use maud::{Markup, html};

// TODO: Implement a 404 page
#[allow(dead_code)]
pub fn not_found() -> Markup {
    let content = html! {
        h1 class="text-3xl font-bold mb-4" {
            "This is not found page"
        }
    };

    base_layout("Page Not found", "Not found", content)
}
