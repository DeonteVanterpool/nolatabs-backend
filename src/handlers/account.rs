use crate::models::account::AutoCommitBehaviour;
use crate::models::account::AutoPullBehaviour;
use crate::models::account::AutoPushBehaviour;
use crate::models::account::CommandStyle;
use crate::models::account::Settings;
use crate::{AppState, logic};
use axum::Extension;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::Result;
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PaymentInfoResponse {
    paid_until: u64,
    subscription_type: String,
}

pub async fn payment_info(
    State(state): State<AppState>,
    uid: Extension<Uuid>,
) -> Result<usize, StatusCode> {
    unimplemented!();
}

#[derive(Serialize, Deserialize)]
pub struct SettingsResponse {
    pub preferred_command_style: String, // unix | plain-english
    pub auto_commit_behaviour: String,   // timer | count | off
    pub auto_commit_timer_interval: u64,
    pub auto_commit_count_interval: u64,
    pub auto_pull_behaviour: String, // timer | on | off
    pub auto_push_behaviour: String, // timer | count | off
    pub auto_push_timer_interval: u64,
    pub auto_push_count_interval: u64,
    pub auto_pull_timer_interval: u64,
}

impl From<Settings> for SettingsResponse {
    fn from(settings: Settings) -> Self {
        let (auto_commit_behaviour, auto_commit_timer_interval, auto_commit_count_interval) =
            match settings.auto_commit_behaviour {
                AutoCommitBehaviour::Timer(delta) => {
                    ("timer".to_string(), delta.num_milliseconds() as u64, 0)
                }
                AutoCommitBehaviour::Count(count) => ("count".to_string(), 0, count as u64),
                AutoCommitBehaviour::Off => ("off".to_string(), 0, 0),
            };

        let (auto_pull_behaviour, auto_pull_timer_interval) = match settings.auto_pull_behaviour {
            AutoPullBehaviour::Timer(delta) => {
                ("timer".to_string(), delta.num_milliseconds() as u64)
            }
            AutoPullBehaviour::On => ("on".to_string(), 0),
            AutoPullBehaviour::Off => ("off".to_string(), 0),
        };

        let (auto_push_behaviour, auto_push_timer_interval, auto_push_count_interval) =
            match settings.auto_push_behaviour {
                AutoPushBehaviour::Timer(delta) => {
                    ("timer".to_string(), delta.num_milliseconds() as u64, 0)
                }
                AutoPushBehaviour::Count(count) => ("count".to_string(), 0, count as u64),
                AutoPushBehaviour::Off => ("off".to_string(), 0, 0),
            };

        SettingsResponse {
            preferred_command_style: settings.preferred_command_style.to_string().to_string(),
            auto_commit_behaviour,
            auto_commit_timer_interval,
            auto_commit_count_interval,
            auto_pull_behaviour,
            auto_push_behaviour,
            auto_push_timer_interval,
            auto_push_count_interval,
            auto_pull_timer_interval,
        }
    }
}

impl From<SettingsResponse> for Settings {
    fn from(response: SettingsResponse) -> Self {
        let preferred_command_style = match response.preferred_command_style.as_str() {
            "unix" => CommandStyle::Unix,
            "plain-english" => CommandStyle::PlainEnglish,
            _ => CommandStyle::Unix, // default fallback
        };

        let auto_commit_behaviour = match response.auto_commit_behaviour.as_str() {
            "timer" => AutoCommitBehaviour::Timer(TimeDelta::milliseconds(
                response.auto_commit_timer_interval as i64,
            )),
            "count" => AutoCommitBehaviour::Count(response.auto_commit_count_interval as u32),
            "off" | _ => AutoCommitBehaviour::Off,
        };

        let auto_pull_behaviour = match response.auto_pull_behaviour.as_str() {
            "timer" => AutoPullBehaviour::Timer(
                TimeDelta::try_milliseconds(response.auto_pull_timer_interval as i64).unwrap(),
            ),
            "on" => AutoPullBehaviour::On,
            "off" | _ => AutoPullBehaviour::Off,
        };

        let auto_push_behaviour = match response.auto_push_behaviour.as_str() {
            "timer" => AutoPushBehaviour::Timer(
                TimeDelta::try_milliseconds(response.auto_push_timer_interval as i64).unwrap(),
            ),
            "count" => AutoPushBehaviour::Count(response.auto_push_count_interval as u32),
            "off" | _ => AutoPushBehaviour::Off,
        };

        Settings {
            preferred_command_style,
            auto_commit_behaviour,
            auto_pull_behaviour,
            auto_push_behaviour,
        }
    }
}

pub async fn get_settings(
    State(state): State<AppState>,
    uid: Extension<Uuid>,
) -> Result<Json<SettingsResponse>, StatusCode> {
    return logic::user::get_settings(state.settings_repository, uid.0)
        .await
        .map_err(|e| e.into())
        .map(SettingsResponse::from)
        .map(Json);
}

pub async fn post_settings(
    State(state): State<AppState>,
    uid: Extension<Uuid>,
    Json(payload): Json<SettingsResponse>,
) -> Result<(), StatusCode> {
    return logic::user::update_settings(state.settings_repository, uid.0, Settings::from(payload))
        .await
        .map_err(|e| e.into());
}
