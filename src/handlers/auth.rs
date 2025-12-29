use crate::AppState;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Result};
use firebase_auth::FirebaseUser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignupPayload {
    name: String,
}

pub async fn init(
    State(state): State<AppState>,
    user: FirebaseUser,
    Json(payload): Json<SignupPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    if !user.email_verified.unwrap_or(false) {
        return Err(StatusCode::FORBIDDEN);
    }
    if user.email.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let uid = state
        .user_repository
        .create_user(&payload.name, &user.email.unwrap());
    return uid.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).map(|v| v.to_string());
}

pub async fn me(State(state): State<AppState>, user: FirebaseUser) -> Result<impl IntoResponse, StatusCode> {
    if !user.email_verified.unwrap_or(false) {
        return Err(StatusCode::FORBIDDEN);
    }
    if user.email.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let uid = state
        .user_repository
        .find_by_email(&user.email.unwrap())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|option| option.ok_or(StatusCode::NOT_FOUND).map(|v| v.to_string()));
    return uid?;
}
