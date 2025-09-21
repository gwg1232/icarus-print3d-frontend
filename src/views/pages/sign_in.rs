use crate::views::layout::base::base_layout;
use maud::{Markup, html};

pub fn sign_in() -> Markup {
    let content = html! {
        div class="max-w-sm mx-auto" {
            h1 class="text-2xl font-bold mb-6 text-center" { "Sign In" }

            form method="POST" action="/sign_in" class="space-y-4" {
                input type="email" name="email" required
                    class="w-full px-3 py-2 border rounded"
                    placeholder="Email";

                input type="password" name="password" required
                    class="w-full px-3 py-2 border rounded"
                    placeholder="Password";

                button type="submit" class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700" {
                    "Sign In"
                }
            }

        }
    };

    base_layout("Sign In", "Sign in", content)
}