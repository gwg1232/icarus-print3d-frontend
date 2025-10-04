use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::{data::errors::DataError, views::pages::server_error};

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("{0}")]
    Data(#[from] DataError),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Data(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, server_error::server_error(&message)).into_response()
    }
}
