use maud::{Markup, html};

// TODO: Remove dead_code allowance when this is used.
#[allow(dead_code)]
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
