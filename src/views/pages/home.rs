use crate::views::components::button::button;
use crate::views::layout::base::base_layout;
use maud::{Markup, html};

pub fn home() -> Markup {
    let content = html! {
        div class="container mx-auto px-4 py-8" {
            (button("Fetch Data", "/fetch-data", "#result"))
        }
    };

    base_layout("Home", content)
}
