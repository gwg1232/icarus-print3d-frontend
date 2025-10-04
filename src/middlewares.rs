use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_sessions::Session;

use crate::auth::{CurrentUser, SESSION_USER_ID_KEY};

pub async fn authenticate(session: Session, mut req: Request, next: Next) -> Response {
    let current_user = match session.get::<i32>(SESSION_USER_ID_KEY).await {
        Ok(Some(user_id)) => CurrentUser::Authenticated { user_id },
        Ok(None) => CurrentUser::Guest,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response(),
    };

    req.extensions_mut().insert(current_user);
    next.run(req).await
}
