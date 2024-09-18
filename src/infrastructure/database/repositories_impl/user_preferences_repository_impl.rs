use crate::domain::entities::user_preferences::UserPreferences;
use crate::domain::repositories::user_preferences_repository::UserPreferencesRepository;
use crate::infrastructure::database::entities::user_preferences;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct UserPreferencesRepositoryImpl {
    pub db: DatabaseConnection,
}

impl UserPreferencesRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserPreferencesRepository for UserPreferencesRepositoryImpl {
    async fn get_preferences_by_user_id(&self, user_id: &str) -> Result<Option<UserPreferences>, Box<dyn std::error::Error>> {
        let pref_model = user_preferences::Entity::find()
            .filter(user_preferences::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?;
        if let Some(pref_model) = pref_model {
            Ok(Some(UserPreferences {
                id: pref_model.id,
                user_id: pref_model.user_id,
                language: pref_model.language,
                night_mode: pref_model.night_mode,
                mail_notification: pref_model.mail_notification,
                sms_notification: pref_model.sms_notification,
                push_notification: pref_model.push_notification,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_preferences(&self, preferences: &UserPreferences) -> Result<(), Box<dyn std::error::Error>> {
        let mut pref_model: user_preferences::ActiveModel = user_preferences::Entity::find()
            .filter(user_preferences::Column::UserId.eq(preferences.user_id.clone()))
            .one(&self.db)
            .await?
            .ok_or("Preferences not found")?
            .into();

        pref_model.language = Set(preferences.language);
        pref_model.night_mode = Set(preferences.night_mode);
        pref_model.mail_notification = Set(preferences.mail_notification);
        pref_model.sms_notification = Set(preferences.sms_notification);
        pref_model.push_notification = Set(preferences.push_notification);

        pref_model.update(&self.db).await?;
        Ok(())
    }
}