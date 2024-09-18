use crate::domain::entities::profile_photo::ProfilePhoto;
use async_trait::async_trait;

#[async_trait]
pub trait ProfilePhotoRepository: Send + Sync {
    async fn get_photo_by_user_name(&self, user_name: &str) -> Result<Option<ProfilePhoto>, Box<dyn std::error::Error>>;
    async fn save_photo(&self, photo: &ProfilePhoto) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_photo_by_user_name(&self, user_name: &str) -> Result<(), Box<dyn std::error::Error>>;
}