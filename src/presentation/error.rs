use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use validator::ValidationErrors;

use crate::application::error::ApplicationError;

#[derive(Serialize)]
pub struct ErrorResponse<T: Serialize = serde_json::Value> {
    pub message: String,
    pub details: Option<T>,
}

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("Payload validation failed")]
    Validation(ValidationErrors),

    #[error("Resource not found")]
    NotFound(String),

    #[error("Conflict")]
    Conflict(String),

    #[error("Unauthorized")]
    Unauthorized(String),

    #[error("Forbidden")]
    Forbidden(String),

    #[error("Internal server error")]
    Internal,
}

impl HttpError {
    pub fn validation(errs: ValidationErrors) -> Self {
        HttpError::Validation(errs)
    }

    fn to_response(&self) -> (StatusCode, Json<ErrorResponse>) {
        match self {
            HttpError::Validation(errs) => {
                let details = serde_json::to_value(errs).unwrap_or(
                    serde_json::json!({"error": "Failed to serialize validation errors"}),
                );
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ErrorResponse {
                        message: "Payload validation failed".to_string(),
                        details: Some(details),
                    }),
                )
            }
            HttpError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: msg.clone(),
                    details: None,
                }),
            ),
            HttpError::Conflict(msg) => (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    message: msg.clone(),
                    details: None,
                }),
            ),
            HttpError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: msg.clone(),
                    details: None,
                }),
            ),
            HttpError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    message: msg.clone(),
                    details: None,
                }),
            ),
            HttpError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Internal server error".to_string(),
                    details: None,
                }),
            ),
        }
    }
}

impl From<ApplicationError> for HttpError {
    fn from(err: ApplicationError) -> Self {
        match err {
            ApplicationError::NotFound(msg) => HttpError::NotFound(msg),

            ApplicationError::Conflict(msg) => HttpError::Conflict(msg),

            ApplicationError::Unauthorized(msg) => HttpError::Unauthorized(msg),

            ApplicationError::Forbidden(msg) => HttpError::Forbidden(msg),

            ApplicationError::Unexpected(e) => {
                tracing::error!("Internal error: {:?}", e);
                HttpError::Internal
            }
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (status, json) = self.to_response();
        (status, json).into_response()
    }
}
