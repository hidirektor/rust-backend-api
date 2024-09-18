use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub id: i32,
    pub user_id: String,
    pub language: bool,
    pub night_mode: bool,
    pub mail_notification: bool,
    pub sms_notification: bool,
    pub push_notification: bool,
}