use crate::models::account::Settings;
use crate::logic::error::ServiceError;
use uuid::Uuid;

use crate::repository::settings::SettingsRepositoryTrait;

pub async fn get_settings<T: SettingsRepositoryTrait>(
    settings_repository: T,
    uid: Uuid,
) -> Result<Settings, ServiceError> {
    return settings_repository.find_by_user_id(uid).await.map_err(|e| e.into());
}

pub async fn update_settings<T: SettingsRepositoryTrait>(
    settings_repository: T,
    uid: Uuid,
    settings: Settings,
) -> Result<(), ServiceError> {
    return settings_repository.update(uid, Settings::from(settings)).await.map_err(|e| e.into());
}

pub fn verify_email(env: crate::state::Environment, email: &str, email_verified: bool) -> bool {
    return email_verified
        || (email.ends_with("@test.account") && env != crate::state::Environment::Production);
}

#[cfg(test)]
mod tests {
    use super::verify_email;
    #[test]
    fn verify_email_tests() {
        assert!(!verify_email(
            crate::state::Environment::Production,
            "fake@outlook.com",
            false
        ));
        assert!(!verify_email(
            crate::state::Environment::Production,
            "fake@test.account",
            false
        ));
        assert!(verify_email(
            crate::state::Environment::Production,
            "fake@outlook.com",
            true
        ));
        assert!(verify_email(
            crate::state::Environment::Production,
            "fake@test.account",
            true
        ));
        assert!(verify_email(
            crate::state::Environment::Testing,
            "fake@test.account",
            true
        ));
        assert!(!verify_email(
            crate::state::Environment::Testing,
            "fake@outlook.com",
            false
        ));
        assert!(verify_email(
            crate::state::Environment::Staging,
            "fake@test.account",
            true
        ));
    }
}
