pub const SESSION_USER_ID_KEY: &str = "authenticated_user_id";

#[derive(Clone, Debug)]
pub enum CurrentUser {
    Authenticated { user_id: i32 },
    Guest,
}

impl CurrentUser {
    pub fn user_id(&self) -> Option<i32> {
        match self {
            Self::Authenticated { user_id } => Some(*user_id),
            Self::Guest => None,
        }
    }
}
