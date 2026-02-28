use thiserror::Error;

use crate::application::error::ApplicationError;

#[derive(Debug, Error)]
pub enum HealthError {
    #[error("An unexpected error occurred")]
    Unexpected(#[from] anyhow::Error),
}

impl From<HealthError> for ApplicationError {
    fn from(err: HealthError) -> Self {
        match err {
            HealthError::Unexpected(e) => ApplicationError::Unexpected(e),
        }
    }
}
