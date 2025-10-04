use crate::data::errors::DataError;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;

pub async fn create_user(db: &PgPool, email: &str, password: &str) -> Result<(), DataError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| DataError::Hash)?
        .to_string();

    sqlx::query!(
        "INSERT INTO users(email, password_hash) VALUES($1, $2)",
        email,
        password_hash.as_bytes()
    )
    .execute(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(e) if e.constraint() == Some("users_email_key") => {
            DataError::DuplicateEmail
        }
        sqlx::Error::Database(e) => DataError::Internal(e.to_string()),
        e => DataError::Query(e),
    })?;

    Ok(())
}
