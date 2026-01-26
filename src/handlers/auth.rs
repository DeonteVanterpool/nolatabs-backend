use crate::AppState;
use crate::repository::user::UserRepositoryTrait;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Result};
use firebase_auth::FirebaseUser;
use serde::{Deserialize, Serialize};

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
        println!("Email not verified and not a test account");
        return Err(StatusCode::FORBIDDEN);
    }

    let uid = state.user_repository.create(&eml).await;
    if uid.is_err() {
        println!("Error creating user: {:?}", uid);
    }
    return uid
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|v| v.to_string());
}

pub async fn me(
    State(state): State<AppState>,
    user: FirebaseUser,
) -> Result<impl IntoResponse, StatusCode> {
    if !user.email_verified.unwrap_or(false) && (!user.email.as_ref().map_or(false, |e| e.ends_with("@test.account")) && state.environment != crate::state::Environment::Production) {
        return Err(StatusCode::FORBIDDEN);
    }
    if user.email.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let uid = state
        .user_repository
        .find_by_email(&user.email.unwrap())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|option| option.ok_or(StatusCode::NOT_FOUND).map(|v| v.to_string()));
    return uid?;
}
