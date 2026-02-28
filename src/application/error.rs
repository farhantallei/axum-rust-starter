use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Not found")]
    NotFound(String),

    #[error("Conflict")]
    Conflict(String),

    #[error("Unauthorized")]
    Unauthorized(String),

    #[error("Forbidden")]
    Forbidden(String),

    #[error("Unexpected error")]
    Unexpected(#[from] anyhow::Error),
}
