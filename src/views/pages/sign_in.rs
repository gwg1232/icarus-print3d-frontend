use crate::{auth::CurrentUser, handlers::dtos::user::{FIELD_EMAIL, FIELD_PASSWORD}, paths, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn sign_in(
    current_user: &CurrentUser,
    email_value: Option<&str>,
    email_error: Option<&str>,
    password_error: Option<&str>,
) -> Markup {
    let content = html! {
        div class="max-w-sm mx-auto" {
            h1 class="text-2xl font-bold mb-6 text-center" { "Sign In" }

            form method="POST" action=(paths::forms::SIGN_IN) class="space-y-4" {
                (render_input("email", FIELD_EMAIL, "Email", email_value, email_error))
                (render_input("password", FIELD_PASSWORD, "Password", None, password_error))

                button type="submit" class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700" {
                    "Sign In"
                }
            }
        }
    };

    base_layout(current_user, "Sign In", "Sign in", content)
}

/// Renders a form input field with optional error styling and message
fn render_input(
    input_type: &str,
    name: &str,
    placeholder: &str,
    value: Option<&str>,
    error: Option<&str>,
) -> Markup {
    let input_class = if error.is_some() {
        "w-full px-3 py-2 border-2 border-red-500 rounded focus:outline-none focus:border-red-600"
    } else {
        "w-full px-3 py-2 border rounded focus:outline-none focus:border-blue-500"
    };

    html! {
        div {
            input type=(input_type) name=(name) required
                class=(input_class)
                placeholder=(placeholder)
                value=[value];

            @if let Some(error_msg) = error {
                p class="mt-1 text-sm text-red-600" { (error_msg) }
            }
        }
    }
}
