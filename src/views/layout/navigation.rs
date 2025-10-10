use crate::{auth::CurrentUser, paths};
use maud::{html, Markup};

pub fn navbar(current_user: &CurrentUser) -> Markup {
    html! {
        header class="bg-white shadow-sm" {
            nav class="container mx-auto px-4" {
                div class="flex justify-between h-16 items-center" {
                    div class="flex gap-6" {
                        a href=(paths::pages::ROOT) class="hover:text-blue-600 transition-colors" { "Home" }
                        @match current_user {
                            CurrentUser::Authenticated { .. } => {
                                a href=(paths::print_orders::BASE) class="hover:text-blue-600 transition-colors font-semibold" { "주문 관리" }
                                a href=(paths::print_orders::new_order::PATH) class="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors" { "새 주문" }
                                a href=(paths::pages::TODOS) class="hover:text-blue-600 transition-colors" { "Todos" }
                                form method="post" action=(paths::actions::SIGN_OUT) class="inline" {
                                    button type="submit" class="hover:text-blue-600 transition-colors" { "Sign Out" }
                                }
                            }
                            CurrentUser::Guest => {
                                a href=(paths::pages::SIGN_UP) class="hover:text-blue-600 transition-colors" { "Sign Up" }
                                a href=(paths::pages::SIGN_IN) class="hover:text-blue-600 transition-colors" { "Sign In" }
                            }
                        }
                    }
                }
            }
        }
    }
}