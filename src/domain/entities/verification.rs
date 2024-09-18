use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Verification {
    pub id: i32,
    pub user_id: String,
    pub mail_verified: bool,
    pub phone_verified: bool,
    pub mail_verification_date: i64,
    pub phone_verification_date: i64,
}