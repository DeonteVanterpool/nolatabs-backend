use uuid::Uuid;
use crate::logic::error::ServiceError;
use crate::repository::user::UserRepositoryTrait;

pub async fn register_user<T: UserRepositoryTrait>(
    user_repository: &T,
    eml: &str,
) -> Result<Uuid, ServiceError> {
    let uid = user_repository.create(&eml).await;
    return uid
        .map_err(|e| ServiceError::from(e));
}

pub async fn login_user<T: UserRepositoryTrait>(
    user_repository: &T,
    eml: &str,
) -> Result<Option<Uuid>, ServiceError> {
    let uid = user_repository.find_by_email(&eml).await;
    return uid
        .map_err(|e| ServiceError::from(e));
}
