use thiserror::Error;

use crate::application::error::ApplicationError;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("An unexpected error occurred")]
    Unexpected(#[from] anyhow::Error),
}

impl From<UserError> for ApplicationError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::NotFound => ApplicationError::NotFound("User not found".into()),

            UserError::Unexpected(e) => ApplicationError::Unexpected(e),
        }
    }
}
