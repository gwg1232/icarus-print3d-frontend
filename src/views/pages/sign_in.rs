use crate::{
    auth::CurrentUser,
    handlers::dtos::user::{FIELD_EMAIL, FIELD_PASSWORD},
    paths,
    views::{components::form, layout::base::base_layout},
};
use maud::{html, Markup};

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
                (form::input("email", FIELD_EMAIL, "Email", email_value, email_error))
                (form::input("password", FIELD_PASSWORD, "Password", None, password_error))
                (form::submit_button("Sign In"))
            }
        }
    };

    base_layout(current_user, "Sign In", "Sign in", content)
}
