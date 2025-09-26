use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

static EMAIL_RX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

#[derive(Deserialize, Validate)]
pub struct SignUpForm {
    #[validate(regex(path = "*EMAIL_RX", message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct SignInForm {
    #[validate(regex(path = "*EMAIL_RX", message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}
