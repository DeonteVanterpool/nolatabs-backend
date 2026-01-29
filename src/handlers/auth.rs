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
    if state.environment == crate::state::Environment::Production {
        if !verified {
            tracing::warn!(email=%eml, "Email not verified");
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        if !verified && !eml.ends_with("@test.account") {
            tracing::warn!(email=%eml, "Email not verified");
            return Err(StatusCode::FORBIDDEN);
        }
    }

    let uid = logic::auth::register_user(
        &state.user_repository,
        &eml,
    ).await;

    return uid
        .map_err(|e| {
            tracing::warn!(email=%eml, error=%e, "Error registering user");
            match e {
                ServiceError::RepositoryError(e) => match e {
                    RepoError::DuplicateEntry(_) => StatusCode::CONFLICT,
                    RepoError::ConnectionError => {
                        tracing::warn!(email=%eml, error=%e, "Error registering user");
                        StatusCode::INTERNAL_SERVER_ERROR
                    }
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })
        .map(|v| v.to_string());
}

pub async fn me(
    uid: Extension<Uuid>
) -> Result<impl IntoResponse, StatusCode> {
    return Ok(uid.to_string());
}
