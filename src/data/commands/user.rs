use crate::data::errors::DataError;
use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use argon2::Argon2;
use sqlx::PgPool;

pub(crate) async fn create_user(db: &PgPool, email: &str, password: &str) -> Result<(), DataError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| DataError::Internal("Password hashing failed"))?
        .to_string();

    sqlx::query!(
        "INSERT INTO users(email, password_hash) VALUES($1, $2)",
        email,
        password_hash
    )
    .execute(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(e) if e.constraint() == Some("users_email_key") => {
            DataError::Conflict("This email address is already used")
        }
        e => DataError::Database(e),
    })?;

    Ok(())
}
