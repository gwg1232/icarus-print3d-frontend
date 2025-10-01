use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Database query failed: {0}")]
    Query(#[from] sqlx::Error),

    #[error("Query failed: {0}")]
    FailedQuery(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Password hashing failed")]
    Hash,
}
