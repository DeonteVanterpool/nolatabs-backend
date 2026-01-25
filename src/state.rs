use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;
use firebase_auth::{FirebaseAuth, FirebaseAuthState};

use crate::repository::user::UserRepository;

#[derive(Clone, PartialEq)]
pub enum Environment {
    Production,
    Staging,
    Testing,
}

#[derive(Clone)]
pub struct AppState {
    // auth: firebase_auth_sdk::Auth,
    pub user_repository: UserRepository,
    pub firebase_auth: FirebaseAuthState,
    pub environment: Environment
}

impl AppState {
    pub fn new(pool: PgPool, firebase_auth: Arc<FirebaseAuth>, environment: Environment) -> Self {
        return AppState {
            user_repository: UserRepository::new(pool),
            firebase_auth: FirebaseAuthState { firebase_auth },
            environment,
        }
    }
}

// Implement FromRef so FirebaseAuthState can be extracted from AppState
impl FromRef<AppState> for FirebaseAuthState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.firebase_auth.clone()
    }
}

