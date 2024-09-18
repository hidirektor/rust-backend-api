use crate::domain::entities::user_preferences::UserPreferences;
use async_trait::async_trait;

#[async_trait]
pub trait UserPreferencesRepository: Send + Sync {
    async fn get_preferences_by_user_id(&self, user_id: &str) -> Result<Option<UserPreferences>, Box<dyn std::error::Error>>;
    async fn update_preferences(&self, preferences: &UserPreferences) -> Result<(), Box<dyn std::error::Error>>;
    // Diğer metotlar...
}