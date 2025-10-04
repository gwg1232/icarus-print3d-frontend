use crate::data::errors::DataError;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::PgPool;

pub(crate) async fn authenticate_user(db: &PgPool, email: &str, password: &str) -> Result<i32, DataError> {
    let row = sqlx::query!(
        "SELECT user_id, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_one(db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => DataError::Unauthorized("Invalid credentials"),
        _ => DataError::Database(e),
    })?;

    let parsed_hash = PasswordHash::new(&row.password_hash).map_err(|_| DataError::Internal("Password hash parsing failed"))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| DataError::Unauthorized("Invalid credentials"))?;

    Ok(row.user_id)
}
