use crate::views::pages;
use maud::Markup;

pub async fn get_root() -> Markup {
    pages::root::root()
}

pub async fn get_about() -> Markup {
    pages::about::about()
}

pub async fn get_create() -> Markup {
    pages::create::create()
}

pub async fn get_sign_up() -> Markup {
    pages::sign_up::sign_up(None, None, None)
}

pub async fn get_sign_in() -> Markup {
    pages::sign_in::sign_in(None, None, None)
}
