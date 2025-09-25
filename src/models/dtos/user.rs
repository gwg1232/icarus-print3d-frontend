use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignInUser {
    pub email: String,
    pub password: String,
}
