use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthService {
    pub user_repository: Arc<dyn UserRepository>,
    pub redis_client: redis::Client,
    pub secret: String,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        redis_client: redis::Client,
        secret: String,
    ) -> Self {
        Self {
            user_repository,
            redis_client,
            secret,
        }
    }

    pub async fn login(&self, user_name: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(user) = self.user_repository.get_user_by_username(user_name).await? {
            if verify(password, &user.password)? {
                let token = self.create_token(&user.user_id)?;
                let mut conn = self.redis_client.get_async_connection().await?;
                conn.set_ex(token.clone(), &user.user_id, 86400).await?; // 1 gün
                Ok(token)
            } else {
                Err("Invalid credentials".into())
            }
        } else {
            Err("User not found".into())
        }
    }

    pub async fn register(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        let hashed_password = hash(&user.password, 4)?;
        let mut new_user = user.clone();
        new_user.password = hashed_password;
        new_user.user_id = Uuid::new_v4().to_string();
        self.user_repository.create_user(&new_user).await?;
        // Hoşgeldin maili gönder
        Ok(())
    }

    pub async fn logout(&self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.redis_client.get_async_connection().await?;
        conn.del(token).await?;
        Ok(())
    }

    fn create_token(&self, user_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        #[derive(Serialize, Deserialize)]
        struct Claims {
            sub: String,
            exp: usize,
        }

        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        Ok(token)
    }
}