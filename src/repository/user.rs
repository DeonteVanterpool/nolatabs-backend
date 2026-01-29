use crate::models::account::AutoCommitBehaviour;
use crate::models::account::AutoPullBehaviour;
use crate::models::account::AutoPushBehaviour;
use crate::models::account::CommandStyle;
use crate::repository::error::RepoError;
use crate::models::account::Settings;
use crate::repository::settings::SettingsRepository;
use crate::repository::settings::SettingsRepositoryTrait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UserRepository {
    conn: PgPool,
    settings: SettingsRepository,
}

pub trait UserRepositoryTrait {
    fn create(&self, email: &str) -> impl Future<Output = Result<Uuid, RepoError>>;
    fn find_by_email(
        &self,
        email: &str,
    ) -> impl Future<Output = Result<Option<Uuid>, RepoError>>;
}

impl UserRepository {
    pub fn new(conn: PgPool) -> Self {
        return Self {
            conn: conn.clone(),
            settings: SettingsRepository::new(conn),
        };
    }
}

impl UserRepositoryTrait for UserRepository {
    async fn create(&self, email: &str) -> Result<Uuid, RepoError> {
        let tx = self.conn.begin().await?;
        let id = sqlx::query!(
            "INSERT INTO users (id, email) VALUES ($1, $2) RETURNING id;",
            Uuid::new_v4(),
            email
        )
        .fetch_one(&self.conn)
        .await.map_err(|e: sqlx::Error| RepoError::from(e))?
        .id;
        let settings = Settings {
            preferred_command_style: CommandStyle::Unix,
            auto_push_behaviour: AutoPushBehaviour::Off,
            auto_pull_behaviour: AutoPullBehaviour::Off,
            auto_commit_behaviour: AutoCommitBehaviour::Off,
        };
        self.settings.create(id, settings).await?;
        tx.commit().await?;
        Ok(id)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Uuid>, RepoError> {
        Ok(sqlx::query!("SELECT id FROM users WHERE email = $1", email)
            .fetch_optional(&self.conn)
            .await.map_err(|e| RepoError::from(e))?
            .map(|v| v.id))
    }
}
