use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

// Field name constants used across forms, validation, and views
// These MUST match the struct field names below for proper deserialization
pub const FIELD_EMAIL: &str = "email";
pub const FIELD_PASSWORD: &str = "password";

static EMAIL_RX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

#[derive(Deserialize, Validate)]
pub struct SignUpForm {
    #[validate(regex(path = "*EMAIL_RX", message = "Invalid email format"))]
    pub email: String,  // Field name must match FIELD_EMAIL constant
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,  // Field name must match FIELD_PASSWORD constant
}

#[derive(Deserialize, Validate)]
pub struct SignInForm {
    #[validate(regex(path = "*EMAIL_RX", message = "Invalid email format"))]
    pub email: String,  // Field name must match FIELD_EMAIL constant
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,  // Field name must match FIELD_PASSWORD constant
}
