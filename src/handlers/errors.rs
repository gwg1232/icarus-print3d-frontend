use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::data::errors::DataError;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Database error")]
    Data(#[from] DataError),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Data(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
