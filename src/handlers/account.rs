use axum::response::IntoResponse;
use crate::AppState;
use crate::models::account::Settings;
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::Result;
use firebase_auth::FirebaseUser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaymentInfoResponse {
    paid_until: u64,
    subscription_type: String,
}

pub async fn payment_info(
    State(state): State<AppState>,
    user: FirebaseUser,
) -> Result<usize, StatusCode> {
    unimplemented!();
}

#[derive(Serialize, Deserialize)]
pub struct SettingsResponse {
    preferred_command_style: String, // unix | plain-english
    auto_commit_behaviour: String, // timer | count | off
    auto_commit_timer_interval: u64,
    auto_commit_count_interval: u64,
    auto_pull_behaviour: String, // timer | on | off
    auto_push_behaviour: String, // timer | conut | off
    auto_push_timer_interval: u64,
    auto_push_count_interval: u64,
}

impl From<Settings> for SettingsResponse {
    fn from(settings: Settings) -> Self {
        todo!();
    }
}

pub async fn get_settings(
    State(state): State<AppState>,
    user: FirebaseUser,
) -> Result<String, StatusCode> {
    todo!();
}

#[axum::debug_handler]
pub async fn post_settings(
    State(state): State<AppState>,
    user: FirebaseUser,
    Json(payload): Json<SettingsResponse>
) -> Result<String, StatusCode> {
    todo!();
}

