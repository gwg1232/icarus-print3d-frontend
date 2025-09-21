use crate::views::layout::base::base_layout;
use crate::paths;
use maud::{html, Markup};

pub fn sign_up() -> Markup {
    let content = html! {
        div class="max-w-sm mx-auto" {
            h1 class="text-2xl font-bold mb-6 text-center" { "Sign Up" }

            form method="POST" action=(paths::forms::SIGN_UP) class="space-y-4" {

                input type="email" name="email" required
                    class="w-full px-3 py-2 border rounded"
                    placeholder="Email";

                input type="password" name="password" required
                    class="w-full px-3 py-2 border rounded"
                    placeholder="Password";

                button type="submit" class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700" {
                    "Sign Up"
                }
            }

        }
    };

    base_layout("Sign Up", "Sign up", content)
}
