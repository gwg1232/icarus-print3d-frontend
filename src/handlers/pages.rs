use crate::views;
use maud::Markup;

pub async fn get_root() -> Markup {
    views::pages::home::home()
}

pub async fn get_about() -> Markup {
    views::pages::about::about()
}

pub async fn get_create() -> Markup {
    views::pages::create::create()
}

pub async fn get_sign_up() -> Markup {
    views::pages::sign_up::sign_up()
}

pub async fn get_sign_in() -> Markup {
    views::pages::sign_in::sign_in()
}