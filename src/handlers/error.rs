use crate::logic::error::ServiceError;
use crate::repository::error::RepoError;
use axum::http::StatusCode;

impl From<ServiceError> for StatusCode {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::RepositoryError(e) => match e {
                RepoError::DuplicateEntry(_) => StatusCode::CONFLICT,
                RepoError::ConnectionError | RepoError::QueryError(_)  => StatusCode::INTERNAL_SERVER_ERROR,
                RepoError::NotFound(_) => StatusCode::NOT_FOUND,
            },
            ServiceError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            ServiceError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            ServiceError::AuthorizationError(_) => StatusCode::FORBIDDEN,
            ServiceError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
