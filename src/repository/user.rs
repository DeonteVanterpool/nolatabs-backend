use uuid::Uuid;
use sqlx::PgPool;
use core::error::Error;

#[derive(Clone, Debug)]
pub struct UserRepository {
    conn: PgPool,
}

impl UserRepository {
    pub fn new(conn: PgPool) -> Self {
        return Self {
            conn,
        }
    }

    pub fn create_user(self, name: &str, email: &str) -> Result<usize, Box<dyn Error>> {
        unimplemented!("TODO: implement get_user_by_email")
    }

    pub async fn find_by_email(self, email: &str) -> Result<Option<Uuid>, Box<dyn Error>> {
        Ok(sqlx::query!("SELECT id FROM users WHERE email = $1", email)
            .fetch_optional(&self.conn).await?.map(|v| v.id))
    }
}
