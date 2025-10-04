use crate::views::layout::base::base_layout;
use maud::{Markup, html};

pub fn server_error(message: &str) -> Markup {
    let content = html! {
        h1 class="text-3xl font-bold mb-4" {
            "Internal server error"
        }
        p class="text-gray-700" {
            (message)
        }
    };

    base_layout("Server error", "Server error", content)
}
