use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, Serialize)]
pub enum ServiceError {
    DbError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct ServiceErrorResponse {
    pub error_msg: String,
}

impl ServiceError {
    fn error_response(&self) -> String {
        match self {
            ServiceError::DbError(msg) => format!("Database error: {msg}"),
            ServiceError::ActixError(msg) => format!("Internal server error: {msg}"),
            ServiceError::NotFound(msg) => msg.to_string(),
            ServiceError::InvalidInput(msg) => msg.to_string(),
        }
    }
}

impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::DbError(_) | ServiceError::ActixError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ServiceErrorResponse {
            error_msg: self.error_response(),
        })
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_response())
    }
}

impl From<sqlx::error::Error> for ServiceError {
    fn from(err: sqlx::error::Error) -> Self {
        ServiceError::DbError(err.to_string())
    }
}

impl From<actix_web::error::Error> for ServiceError {
    fn from(err: actix_web::error::Error) -> Self {
        ServiceError::ActixError(err.to_string())
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(err: std::io::Error) -> Self {
        ServiceError::ActixError(err.to_string())
    }
}
