use sqlx::PgPool;

use crate::repository::user::UserRepository;

#[derive(Debug, Clone)]
pub struct AppState {
    // auth: firebase_auth_sdk::Auth,
    pub user_repository: UserRepository,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        return AppState {
            user_repository: UserRepository::new(pool)
        }
    }
}
