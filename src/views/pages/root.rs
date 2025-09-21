use crate::views::layout::base::base_layout;
use maud::{Markup, html};

pub fn root() -> Markup {
    let content = html! {
        h1 class="text-3xl font-bold mb-4" {
            "This is home page"
        }
    };

    base_layout("Home", "Home", content)
}
