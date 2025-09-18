use maud::{html, Markup};

pub fn navbar() -> Markup {
    html! {
        nav class="bg-white shadow-sm" {
            div class="container mx-auto px-4" {
                div class="flex justify-between h-16 items-center" {
                    a href="/" class="text-xl font-bold" {
                        "App"
                    }

                    div class="flex gap-4" {
                        a href="/" class="hover:text-blue-600" { "Home" }
                        a href="/about" class="hover:text-blue-600" { "About" }
                    }
                }
            }
        }
    }
}