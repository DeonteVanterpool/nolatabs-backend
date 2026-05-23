use uuid::Uuid;
use sqlx::PgPool;
use crate::repository::error::RepoError;

#[derive(Clone, Debug)]
pub struct RepoRepository {
    conn: PgPool,
}

pub trait RepoRepositoryTrait {
    fn create(&self, owner_id: Uuid, name: String, owner_name: String) -> impl Future<Output = Result<(), RepoError>>;
}

impl RepoRepository {
    pub fn new(conn: PgPool) -> Self {
        return Self {
            conn: conn.clone(),
        };
    }
}

impl RepoRepositoryTrait for RepoRepository {
    async fn create(&self, owner_id: Uuid, name: String, owner_name: String) -> Result<(), RepoError> {
        let tx = self.conn.begin().await?;
        let id = sqlx::query!(
            "INSERT INTO repos (id, owner, name) VALUES ($1, $2, $3);",
            owner_name + "/" + &name,
            owner_id,
            name,
        ).execute(&self.conn)
        .await.map_err(|e: sqlx::Error| RepoError::from(e))?;
        tx.commit().await?;
        Ok(())
    }
}
