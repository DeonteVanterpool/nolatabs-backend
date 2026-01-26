use crate::AppState;
use crate::repository::user::UserRepositoryTrait;
use axum::Extension;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Result};
use firebase_auth::FirebaseUser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SignupPayload {}

pub async fn init(
    State(state): State<AppState>,
    user: FirebaseUser,
    // Json(payload): Json<SignupPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    if user.email.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let eml = user.email.unwrap();
    if !user.email_verified.unwrap_or(false) && (!&eml.ends_with("@test.account") && state.environment != crate::state::Environment::Production) {
        // email not verified AND not a test account
        tracing::warn!(email=%eml, "Email not verified");
        return Err(StatusCode::FORBIDDEN);
    }

    let uid = state.user_repository.create(&eml).await;
    if uid.is_err() {
        tracing::error!(email=%eml, error=%uid.as_ref().err().unwrap(), "Database error creating user");
    }
    return uid
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|v| v.to_string());
}

pub async fn me(
    uid: Extension<Uuid>
) -> Result<impl IntoResponse, StatusCode> {
    return Ok(uid.to_string());
}
