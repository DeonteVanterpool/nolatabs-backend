use crate::repository::error::RepoError;
use crate::models::account::AutoCommitBehaviour;
use crate::models::account::AutoPullBehaviour;
use crate::models::account::AutoPushBehaviour;
use crate::models::account::CommandStyle;
use crate::models::account::Settings;
use sqlx::PgPool;
use sqlx::postgres::types::PgInterval;
use std::error::Error;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SettingsRepository {
    conn: PgPool,
}

pub trait SettingsRepositoryTrait {
    fn create(
        &self,
        uid: Uuid,
        settings: SettingsParams,
    ) -> impl Future<Output = Result<(), RepoError>>;
    fn update(
        &self,
        uid: Uuid,
        settings: SettingsParams,
    ) -> impl Future<Output = Result<(), RepoError>>;
    fn find_by_user_id(
        &self,
        uid: Uuid,
    ) -> impl Future<Output = Result<Settings, RepoError>>;
}

impl SettingsRepository {
    pub fn new(conn: PgPool) -> Self {
        return Self { conn };
    }
}

pub struct SettingsParams {
    pub preferred_command_style: CommandStyle,
    pub auto_commit_behaviour: AutoCommitBehaviour,
    pub auto_pull_behaviour: AutoPullBehaviour,
    pub auto_push_behaviour: AutoPushBehaviour,
}

fn to_timedelta(duration: PgInterval) -> chrono::TimeDelta {
    // Convert PgInterval to chrono::TimeDelta
    let months = duration.months as i64;
    let days = duration.days as i64;
    let microseconds = duration.microseconds as i64;
    let total_days = months * 30 + days; // Approximate months to days
    return chrono::Duration::days(total_days) + chrono::Duration::microseconds(microseconds);
}

impl SettingsRepositoryTrait for SettingsRepository {
    async fn create(&self, user_id: Uuid, settings: SettingsParams) -> Result<(), RepoError> {
        let cmd_style = match settings.preferred_command_style {
            CommandStyle::Unix => "terminal style",
            CommandStyle::PlainEnglish => "plain-english style",
        };
        let (autopush_option, autopush_duration, autopush_interval_count): (
            _,
            Option<PgInterval>,
            _,
        ) = match settings.auto_push_behaviour {
            AutoPushBehaviour::Timer(d) => ("timer", Some(d.try_into().unwrap()), None),
            AutoPushBehaviour::Count(i) => ("count", None, Some(i as i32)),
            AutoPushBehaviour::Off => ("off", None, None),
        };
        let (autopull_option, autopull_duration): (_, Option<PgInterval>) =
            match settings.auto_pull_behaviour {
                AutoPullBehaviour::On => ("on", None),
                AutoPullBehaviour::Timer(d) => ("timer", Some(d.try_into().unwrap())),
                AutoPullBehaviour::Off => ("off", None),
            };
        let (autocommit_option, autocommit_duration, autocommit_interval_count): (
            _,
            Option<PgInterval>,
            _,
        ) = match settings.auto_commit_behaviour {
            AutoCommitBehaviour::Timer(d) => ("timer", Some(d.try_into().unwrap()), None),
            AutoCommitBehaviour::Count(i) => ("count", None, Some(i as i32)),
            AutoCommitBehaviour::Off => ("off", None, None),
        };
        sqlx::query!("INSERT INTO user_settings
(user_id, command_style, autopush_option, autopush_duration, autopush_interval_count, autopull_option, autopull_duration, autocommit_option, autocommit_duration, autocommit_interval_count)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)", user_id, cmd_style, autopush_option, autopush_duration, autopush_interval_count, autopull_option, autopull_duration, autocommit_option, autocommit_duration, autocommit_interval_count)
            .execute(&self.conn)
            .await.map_err(|e| RepoError::Database(e))?;
        Ok(())
    }

    async fn update(&self, user_id: Uuid, settings: SettingsParams) -> Result<(), RepoError> {
        let cmd_style = match settings.preferred_command_style {
            CommandStyle::Unix => "terminal style",
            CommandStyle::PlainEnglish => "plain-english style",
        };
        let (autopush_option, autopush_duration, autopush_interval_count): (
            _,
            Option<PgInterval>,
            _,
        ) = match settings.auto_push_behaviour {
            AutoPushBehaviour::Timer(d) => ("timer", Some(d.try_into().unwrap()), None),
            AutoPushBehaviour::Count(i) => ("count", None, Some(i as i32)),
            AutoPushBehaviour::Off => ("off", None, None),
        };
        let (autopull_option, autopull_duration): (_, Option<PgInterval>) =
            match settings.auto_pull_behaviour {
                AutoPullBehaviour::On => ("on", None),
                AutoPullBehaviour::Timer(d) => ("timer", Some(d.try_into().unwrap())),
                AutoPullBehaviour::Off => ("off", None),
            };
        let (autocommit_option, autocommit_duration, autocommit_interval_count): (
            _,
            Option<PgInterval>,
            _,
        ) = match settings.auto_commit_behaviour {
            AutoCommitBehaviour::Timer(d) => ("timer", Some(d.try_into().unwrap()), None),
            AutoCommitBehaviour::Count(i) => ("count", None, Some(i as i32)),
            AutoCommitBehaviour::Off => ("off", None, None),
        };
        // COALESCE is used to prevent overwriting existing durations/counts with NULL when the
        // option is changed
        sqlx::query!("UPDATE user_settings SET command_style = $2, autopush_option = $3, autopush_duration = COALESCE($4, autopush_duration), autopush_interval_count = COALESCE($5, autopush_interval_count), autopull_option = $6, autopull_duration = COALESCE($7, autopull_duration), autocommit_option = $8, autocommit_duration = COALESCE($9, autocommit_duration), autocommit_interval_count = COALESCE($10, autocommit_interval_count) WHERE user_id = $1", user_id, cmd_style, autopush_option, autopush_duration, autopush_interval_count, autopull_option, autopull_duration, autocommit_option, autocommit_duration, autocommit_interval_count)
            .execute(&self.conn)
            .await.map_err(|e| RepoError::Database(e))?;
        Ok(())
    }

    async fn find_by_user_id(&self, uid: Uuid) -> Result<Settings, RepoError> {
        let record = sqlx::query!("SELECT * FROM user_settings WHERE user_id = $1", uid)
            .fetch_optional(&self.conn)
            .await.map_err(|e| RepoError::Database(e))?;
        if let Some(rec) = record {
            let preferred_command_style = match rec.command_style.as_str() {
                "terminal style" => CommandStyle::Unix,
                "plain-english style" => CommandStyle::PlainEnglish,
                _ => CommandStyle::Unix,
            };
            let auto_push_behaviour = match rec.autopush_option.as_str() {
                "timer" => AutoPushBehaviour::Timer(to_timedelta(rec.autopush_duration.unwrap())),
                "count" => AutoPushBehaviour::Count(rec.autopush_interval_count.unwrap() as u32),
                "off" => AutoPushBehaviour::Off,
                _ => AutoPushBehaviour::Off,
            };
            let auto_pull_behaviour = match rec.autopull_option.as_str() {
                "on" => AutoPullBehaviour::On,
                "timer" => {
                    AutoPullBehaviour::Timer(to_timedelta(rec.autopull_duration.unwrap()))
                }
                "off" => AutoPullBehaviour::Off,
                _ => AutoPullBehaviour::Off,
            };
            let auto_commit_behaviour = match rec.autocommit_option.as_str() {
                "timer" => {
                    AutoCommitBehaviour::Timer(to_timedelta(rec.autocommit_duration.unwrap()))
                }
                "count" => {
                    AutoCommitBehaviour::Count(rec.autocommit_interval_count.unwrap() as u32)
                }
                "off" => AutoCommitBehaviour::Off,
                _ => AutoCommitBehaviour::Off,
            };
            Ok(Settings {
                preferred_command_style,
                auto_push_behaviour,
                auto_pull_behaviour,
                auto_commit_behaviour,
            })
        } else {
            return Err(RepoError::NotFound("Settings not found".to_string()));
        }
    }
}
