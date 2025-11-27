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

    pub fn find_by_email(self, email: &str) -> Result<Option<usize>, Box<dyn Error>> {
        unimplemented!("TODO: implement get_user_by_email")
    }
}
