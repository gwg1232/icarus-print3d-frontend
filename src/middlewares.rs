use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;

use crate::{auth::{CurrentUser, SESSION_USER_ID_KEY}, flash::FlashMessage, paths};

pub async fn session_context(session: Session, mut req: Request, next: Next) -> Response {
    let current_user = match session.get::<i32>(SESSION_USER_ID_KEY).await {
        Ok(Some(user_id)) => CurrentUser::Authenticated { user_id },
        Ok(None) => CurrentUser::Guest,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response(),
    };

    let flash = match FlashMessage::get(&session).await {
        Ok(flash) => flash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response(),
    };

    req.extensions_mut().insert(current_user);
    req.extensions_mut().insert(flash);
    next.run(req).await
}

pub async fn require_authentication(req: Request, next: Next) -> Response {
    match req.extensions().get::<CurrentUser>() {
        Some(CurrentUser::Authenticated { .. }) => {
            let mut res = next.run(req).await;
            res.headers_mut().insert(
                header::CACHE_CONTROL,
                "no-store, no-cache, must-revalidate, private".parse().unwrap(),
            );
            res
        }
        _ => {
            let session = req.extensions().get::<Session>().cloned();
            if let Some(session) = session {
                let _ = FlashMessage::error("Please sign in to continue").set(&session).await;
            }
            Redirect::to(paths::pages::SIGN_IN).into_response()
        }
    }
}
