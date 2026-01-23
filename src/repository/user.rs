use core::error::Error;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::account::Settings;

#[derive(Clone, Debug)]
pub struct UserRepository {
    conn: PgPool,
}

pub trait UserRepositoryTrait {
    fn create(&self, email: &str) -> impl Future<Output = Result<Uuid, Box<dyn Error>>>;
    fn find_by_email(
        &self,
        email: &str,
    ) -> impl Future<Output = Result<Option<Uuid>, Box<dyn Error>>>;
}

impl UserRepository {
    pub fn new(conn: PgPool) -> Self {
        return Self { conn };
    }
}

impl UserRepositoryTrait for UserRepository {
    async fn create(&self, email: &str) -> Result<Uuid, Box<dyn Error>> {
        let tx = self.conn.begin().await?;
        let id = sqlx::query!(
            "INSERT INTO users (id, email) VALUES ($1, $2) RETURNING id;",
            Uuid::new_v4(),
            email
        )
        .fetch_one(&self.conn)
        .await?
        .id;
        tx.commit().await?;
        Ok(id)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Uuid>, Box<dyn Error>> {
        Ok(sqlx::query!("SELECT id FROM users WHERE email = $1", email)
            .fetch_optional(&self.conn)
            .await?
            .map(|v| v.id))
    }
}

/*
async fn create_settings(&conn: &PgPool, user: Uuid) -> Result<(), Box<dyn Error>> {
    let settings = Settings {
        preferred_command_style: todo!(),
        auto_commit_behaviour: todo!(),
        auto_pull_behaviour: todo!(),
        auto_push_behaviour: todo!(),
    };
    let preferred_command_style = settings.preferred_command_style.to_string();
    return sqlx::query!(
        "INSERT INTO user_settings VALUES ($1, $2, $3, $4, $5, $6, $7)",
        user,
        preferred_command_style,
        settings.auto_push_behaviour
    )
    .fetch_one(conn)
    .await?;
}
*/
