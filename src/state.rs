use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;
use firebase_auth::{FirebaseAuth, FirebaseAuthState};

use crate::repository::user::UserRepository;

#[derive(Clone)]
pub struct AppState {
    // auth: firebase_auth_sdk::Auth,
    pub user_repository: UserRepository,
    pub firebase_auth: FirebaseAuthState,
}

impl AppState {
    pub fn new(pool: PgPool, firebase_auth: Arc<FirebaseAuth>) -> Self {
        return AppState {
            user_repository: UserRepository::new(pool),
            firebase_auth: FirebaseAuthState { firebase_auth },
        }
    }
}

// Implement FromRef so FirebaseAuthState can be extracted from AppState
impl FromRef<AppState> for FirebaseAuthState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.firebase_auth.clone()
    }
}

