use super::error::RepoError;
use crate::models::account::Payment;
use uuid::Uuid;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct PaymentLogRepository {
    conn: PgPool,
}

pub trait PaymentLogRepositoryTrait {
    fn create(
        &self,
        sub_info: Payment,
    ) -> impl Future<Output = Result<(), RepoError>>;
    fn find_by_user_id(
        &self,
        uid: Uuid,
    ) -> impl Future<Output = Result<Vec<Payment>, RepoError>>;
}

impl PaymentLogRepository {
    pub fn new(conn: PgPool) -> Self {
        return Self { conn };
    }
}

impl PaymentLogRepositoryTrait for PaymentLogRepository {
    async fn create(&self, payment_info: Payment) -> Result<(), RepoError> {
        sqlx::query!("INSERT INTO payment_log (payment_id, user_id, amount_cents, payment_date) VALUES ($1, $2, $3, $4)", payment_info.payment_id, payment_info.user_id, payment_info.amount_cents as i32, payment_info.payment_date)
            .execute(&self.conn)
            .await.map_err(|e| RepoError::from(e))?;
        Ok(())
    }

    async fn find_by_user_id(&self, uid: Uuid) -> Result<Vec<Payment>, RepoError> {
        let record = sqlx::query!("SELECT * FROM payment_log WHERE user_id = $1", uid)
            .fetch_all(&self.conn)
            .await.map_err(|e| RepoError::from(e))?;
        let payments = record.into_iter().map(|rec| Payment {
            amount_cents: rec.amount_cents as u32,
            payment_id: rec.payment_id,
            payment_date: rec.payment_date,
            user_id: rec.user_id,
        }).collect();
        Ok(payments)
    }
}

