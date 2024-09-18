use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::database::entities::user;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct UserRepositoryImpl {
    pub db: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let active_model = user::ActiveModel {
            user_id: Set(user.user_id.clone()),
            user_name: Set(user.user_name.clone()),
            name_surname: Set(user.name_surname.clone()),
            email: Set(user.email.clone()),
            phone_number: Set(user.phone_number.clone()),
            company_name: Set(user.company_name.clone()),
            password: Set(user.password.clone()),
            is_active: Set(user.is_active),
            ..Default::default()
        };
        user::Entity::insert(active_model).exec(&self.db).await?;
        Ok(())
    }

    async fn get_user_by_username(&self, user_name: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let user_model = user::Entity::find()
            .filter(user::Column::UserName.eq(user_name))
            .one(&self.db)
            .await?;
        if let Some(user_model) = user_model {
            Ok(Some(User {
                id: user_model.id,
                user_id: user_model.user_id,
                user_name: user_model.user_name,
                name_surname: user_model.name_surname,
                email: user_model.email,
                phone_number: user_model.phone_number,
                company_name: user_model.company_name,
                password: user_model.password,
                is_active: user_model.is_active,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_user(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let mut user_model: user::ActiveModel = user::Entity::find()
            .filter(user::Column::UserId.eq(user.user_id.clone()))
            .one(&self.db)
            .await?
            .ok_or("User not found")?
            .into();

        user_model.name_surname = Set(user.name_surname.clone());
        user_model.email = Set(user.email.clone());
        user_model.phone_number = Set(user.phone_number.clone());
        user_model.company_name = Set(user.company_name.clone());
        user_model.password = Set(user.password.clone());

        user_model.update(&self.db).await?;
        Ok(())
    }
}