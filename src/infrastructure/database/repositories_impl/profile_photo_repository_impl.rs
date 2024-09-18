use crate::domain::entities::profile_photo::ProfilePhoto;
use crate::domain::repositories::profile_photo_repository::ProfilePhotoRepository;
use crate::infrastructure::database::entities::profile_photo;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct ProfilePhotoRepositoryImpl {
    pub db: DatabaseConnection,
}

impl ProfilePhotoRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProfilePhotoRepository for ProfilePhotoRepositoryImpl {
    async fn get_photo_by_user_name(&self, user_name: &str) -> Result<Option<ProfilePhoto>, Box<dyn std::error::Error>> {
        let photo_model = profile_photo::Entity::find()
            .filter(profile_photo::Column::UserName.eq(user_name))
            .one(&self.db)
            .await?;
        if let Some(photo_model) = photo_model {
            Ok(Some(ProfilePhoto {
                id: photo_model.id,
                user_id: photo_model.user_id,
                user_name: photo_model.user_name,
                file_id: photo_model.file_id,
                upload_date: photo_model.upload_date,
            }))
        } else {
            Ok(None)
        }
    }

    async fn save_photo(&self, photo: &ProfilePhoto) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = profile_photo::ActiveModel {
            user_id: Set(photo.user_id.clone()),
            user_name: Set(photo.user_name.clone()),
            file_id: Set(photo.file_id.clone()),
            upload_date: Set(photo.upload_date),
            ..Default::default()
        };
        profile_photo::Entity::insert(active_model).exec(&self.db).await?;
        Ok(())
    }

    async fn delete_photo_by_user_name(&self, user_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        profile_photo::Entity::delete_many()
            .filter(profile_photo::Column::UserName.eq(user_name))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}