use crate::logic::error::ServiceError;
use crate::repository::error::RepoError;
use crate::{AppState, logic};
use axum::Extension;
use axum::extract::State;
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
    let verified = user.email_verified.unwrap_or(false);
    crate::logic::user::verify_email(state.environment, &eml, verified);

    let uid = logic::auth::register_user(
        &state.user_repository,
        &eml,
    ).await;

    return uid
        .map_err(|e| {
            tracing::warn!(email=%eml, error=%e, "Error registering user");
            e.into()
        })
        .map(|v| v.to_string());
}

pub async fn me(
    uid: Extension<Uuid>
) -> Result<impl IntoResponse, StatusCode> {
    return Ok(uid.to_string());
}
