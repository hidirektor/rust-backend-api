use crate::domain::entities::verification::Verification;
use async_trait::async_trait;

#[async_trait]
pub trait VerificationRepository: Send + Sync {
    async fn get_verification_by_user_id(&self, user_id: &str) -> Result<Option<Verification>, Box<dyn std::error::Error>>;
    async fn update_verification(&self, verification: &Verification) -> Result<(), Box<dyn std::error::Error>>;
}