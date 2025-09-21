use maud::{html, Markup};

pub fn navbar() -> Markup {
    html! {
        header class="bg-white shadow-sm" {
            nav class="container mx-auto px-4" {
                div class="flex justify-between h-16 items-center" {
                    div class="flex gap-6" {
                        a href="/" class="hover:text-blue-600 transition-colors" { "Home" }
                        a href="/create" class="hover:text-blue-600 transition-colors" { "Create" }
                        a href="/todos" class="hover:text-blue-600 transition-colors" { "Todos" }
                        a href="/sign_up" class="hover:text-blue-600 transition-colors" { "Sign Up" }
                        a href="/sign_in" class="hover:text-blue-600 transition-colors" { "Sign In" }
                    }
                }
            }
        }
    }
}