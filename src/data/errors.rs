use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[allow(dead_code)]
    #[error("{0}")]
    NotFound(&'static str),

    #[error("{0}")]
    Unauthorized(&'static str),

    #[error("{0}")]
    Conflict(&'static str),

    #[error("{0}")]
    Internal(&'static str),
}
