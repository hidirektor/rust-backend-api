use crate::domain::entities::verification::Verification;
use crate::domain::repositories::verification_repository::VerificationRepository;
use crate::infrastructure::database::entities::verification;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct VerificationRepositoryImpl {
    pub db: DatabaseConnection,
}

impl VerificationRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl VerificationRepository for VerificationRepositoryImpl {
    async fn get_verification_by_user_id(&self, user_id: &str) -> Result<Option<Verification>, Box<dyn std::error::Error>> {
        let ver_model = verification::Entity::find()
            .filter(verification::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?;
        if let Some(ver_model) = ver_model {
            Ok(Some(Verification {
                id: ver_model.id,
                user_id: ver_model.user_id,
                mail_verified: ver_model.mail_verified,
                phone_verified: ver_model.phone_verified,
                mail_verification_date: ver_model.mail_verification_date,
                phone_verification_date: ver_model.phone_verification_date,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_verification(&self, verification: &Verification) -> Result<(), Box<dyn std::error::Error>> {
        let mut ver_model: verification::ActiveModel = verification::Entity::find()
            .filter(verification::Column::UserId.eq(verification.user_id.clone()))
            .one(&self.db)
            .await?
            .ok_or("Verification not found")?
            .into();

        ver_model.mail_verified = Set(verification.mail_verified);
        ver_model.phone_verified = Set(verification.phone_verified);
        ver_model.mail_verification_date = Set(verification.mail_verification_date);
        ver_model.phone_verification_date = Set(verification.phone_verification_date);

        ver_model.update(&self.db).await?;
        Ok(())
    }
}