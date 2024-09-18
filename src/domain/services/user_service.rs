use crate::domain::entities::{user::User, user_preferences::UserPreferences};
use crate::domain::repositories::{user_preferences_repository::UserPreferencesRepository, user_repository::UserRepository};
use std::sync::Arc;

pub struct UserService {
    pub user_repository: Arc<dyn UserRepository>,
    pub user_preferences_repository: Arc<dyn UserPreferencesRepository>,
    pub redis_client: redis::Client,
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        user_preferences_repository: Arc<dyn UserPreferencesRepository>,
        redis_client: redis::Client,
    ) -> Self {
        Self {
            user_repository,
            user_preferences_repository,
            redis_client,
        }
    }

    pub async fn get_profile(&self, user_id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        // Kullanıcı profilini döndür
        // ...
        Ok(None)
    }

    pub async fn update_profile(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        // Kullanıcı profilini güncelle
        // ...
        Ok(())
    }

    pub async fn change_password(&self, user_name: &str, old_password: &str, new_password: &str, close_sessions: bool) -> Result<(), Box<dyn std::error::Error>> {
        // Şifre değiştirme işlemleri
        // ...
        Ok(())
    }

    pub async fn update_preferences(&self, preferences: UserPreferences) -> Result<(), Box<dyn std::error::Error>> {
        self.user_preferences_repository.update_preferences(&preferences).await?;
        Ok(())
    }
}