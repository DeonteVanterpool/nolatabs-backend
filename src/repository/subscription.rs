use crate::models::account::SubscriptionType;
use crate::models::account::SubscriptionInfo;
use crate::repository::error::RepoError;
use crate::models::account::AutoCommitBehaviour;
use crate::models::account::AutoPullBehaviour;
use crate::models::account::AutoPushBehaviour;
use crate::models::account::CommandStyle;
use crate::models::account::Settings;
use sqlx::PgPool;
use sqlx::postgres::types::PgInterval;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SubscriptionRepository {
    conn: PgPool,
}

pub trait SubscriptionRepositoryTrait {
    fn create(
        &self,
        uid: Uuid,
        sub_info: SubscriptionInfo,
    ) -> impl Future<Output = Result<(), RepoError>>;
    fn update(
        &self,
        uid: Uuid,
        sub_info: SubscriptionInfo,
    ) -> impl Future<Output = Result<(), RepoError>>;
    fn find_by_user_id(
        &self,
        uid: Uuid,
    ) -> impl Future<Output = Result<SubscriptionInfo, RepoError>>;
}

impl SubscriptionRepository {
    pub fn new(conn: PgPool) -> Self {
        return Self { conn };
    }
}

impl SubscriptionRepositoryTrait for SubscriptionRepository {
    async fn create(&self, user_id: Uuid, sub_info: SubscriptionInfo) -> Result<(), RepoError> {
        sqlx::query!("INSERT INTO credit (user_id, plan_id, paid_until) VALUES ($1, $2, $3)", user_id, sub_info.subscription_type.to_string(), sub_info.paid_until)
            .execute(&self.conn)
            .await.map_err(|e| RepoError::from(e))?;
        Ok(())
    }

    async fn update(&self, user_id: Uuid, sub_info: SubscriptionInfo) -> Result<(), RepoError> {
        sqlx::query!("UPDATE credit SET plan_id = $2, paid_until = $3 WHERE user_id = $1", user_id, sub_info.subscription_type.to_string(), sub_info.paid_until)
            .execute(&self.conn)
            .await.map_err(|e| RepoError::from(e))?;
        Ok(())
    }

    async fn find_by_user_id(&self, uid: Uuid) -> Result<SubscriptionInfo, RepoError> {
        let record = sqlx::query!("SELECT * FROM credit WHERE user_id = $1", uid)
            .fetch_optional(&self.conn)
            .await.map_err(|e| RepoError::from(e))?;
        if let Some(rec) = record {
            let subscription_type = SubscriptionType::from_string(&rec.plan_id)
                .ok_or_else(|| RepoError::NotFound("Invalid subscription type".to_string()))?;
            let paid_until = rec.paid_until;
            Ok(SubscriptionInfo {
                paid_until,
                subscription_type,
            })
        } else {
            return Err(RepoError::NotFound("Settings not found".to_string()));
        }
    }
}
