use crate::views::layout::base::base_layout;
use maud::{Markup, html};

// TODO: Implement a 500 page
#[allow(dead_code)]
pub fn server_error() -> Markup {
    let content = html! {
        h1 class="text-3xl font-bold mb-4" {
            "Internal server error"
        }
    };

    base_layout("Server error", "Server error", content)
}
