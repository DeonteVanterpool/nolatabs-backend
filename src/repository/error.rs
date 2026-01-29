use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Database connection error")]
    ConnectionError,

    #[error("Duplicate entry error: {0}")]
    DuplicateEntry(String),

    #[error("Database query error: {0}")]
    QueryError(String),
}

impl From<sqlx::Error> for RepoError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => RepoError::NotFound("row not found".to_string()),
            sqlx::Error::PoolTimedOut => RepoError::ConnectionError,
            sqlx::Error::Database(db_err) if db_err.code() == Some("23505".into()) => {
                RepoError::DuplicateEntry(db_err.message().to_string())
            }
            _ => RepoError::QueryError(err.to_string()),
        }
    }
}
