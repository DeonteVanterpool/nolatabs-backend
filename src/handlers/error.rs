use crate::logic::error::ServiceError;
use crate::repository::error::RepoError;
use reqwest::StatusCode;

impl From<ServiceError> for StatusCode {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::RepositoryError(e) => match e {
                RepoError::DuplicateEntry(_) => StatusCode::CONFLICT,
                RepoError::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
