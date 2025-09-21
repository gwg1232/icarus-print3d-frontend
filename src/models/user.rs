use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignInUser {
    pub email: String,
    pub password: String,
}
