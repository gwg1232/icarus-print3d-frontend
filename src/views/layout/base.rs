use maud::{DOCTYPE, Markup, html};
use super::{navigation, footer};

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }

                // Tailwind CSS CDN
                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}

                // HTMX CDN
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js"
                    integrity="sha384-ZBXiYtYQ6hJ2Y0ZNoYuI+Nq5MqWBr+chMrS/RkXpNzQCApHEhOt2aY8EJgqwHLkJ"
                    crossorigin="anonymous" {}

                // Custom styles or scripts can be added here
            }
            body class="bg-gray-50 text-gray-900 min-h-screen flex flex-col" {
                (navigation::navbar())
                main class="flex-grow" {
                    (content)
                }
                (footer::footer())
            }
        }
    }
}