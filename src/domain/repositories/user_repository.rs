use crate::domain::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_user_by_username(&self, user_name: &str) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn update_user(&self, user: &User) -> Result<(), Box<dyn std::error::Error>>;
    // Diğer metotlar...
}