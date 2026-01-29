use thiserror::Error;
use crate::repository::error::RepoError;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Database error: {0}")]
    RepositoryError(#[from] RepoError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

