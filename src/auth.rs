pub const SESSION_USER_ID_KEY: &str = "authenticated_user_id";

#[derive(Clone, Debug)]
pub enum CurrentUser {
    Authenticated {
        #[allow(dead_code)]
        user_id: i32,
    },
    Guest,
}

