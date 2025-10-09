pub const SESSION_USER_ID_KEY: &str = "authenticated_user_id";

#[derive(Clone, Debug)]
pub enum CurrentUser {
    Authenticated { user_id: i32 },
    Guest,
}

impl CurrentUser {
    pub fn require_authenticated(&self) -> i32 {
        match self {
            CurrentUser::Authenticated { user_id } => *user_id,
            CurrentUser::Guest => unreachable!("Protected route accessed by guest user"),
        }
    }
}

