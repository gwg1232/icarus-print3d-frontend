use maud::{html, Markup};

pub fn footer() -> Markup {
    html! {
        footer class="bg-gray-100 mt-auto" {
            div class="container mx-auto px-4 py-4" {
                p class="text-center text-sm text-gray-600" {
                    "© 2024 - Built with Rust"
                }
            }
        }
    }
}