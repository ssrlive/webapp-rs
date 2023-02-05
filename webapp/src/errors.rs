use serde::Serialize;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum AppError {
    ActixError(String),
    NotFound(String),
    TemplateError,
}

#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    pub error_message: String,
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn error_response(&self) -> String {
        match self {
            AppError::ActixError(e) => e.to_string(),
            AppError::NotFound(e) => e.to_string(),
            AppError::TemplateError => "Template Error".to_string(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<actix_web::error::Error> for AppError {
    fn from(e: actix_web::error::Error) -> Self {
        AppError::ActixError(e.to_string())
    }
}
