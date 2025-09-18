use maud::{Markup, html};

pub fn button(text: &str, hx_get: &str, target: &str) -> Markup {
    html! {
        button
            hx-get=(hx_get)
            hx-target=(target)
            class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded" {
            (text)
        }
    }
}
