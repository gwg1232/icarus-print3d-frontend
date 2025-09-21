use crate::views;
use maud::Markup;

pub async fn home() -> Markup {
    views::pages::home::home()
}

pub async fn about() -> Markup {
    views::pages::about::about()
}

pub async fn create() -> Markup {
    views::pages::create::create()
}