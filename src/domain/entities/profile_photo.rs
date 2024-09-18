use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfilePhoto {
    pub id: i32,
    pub user_id: String,
    pub user_name: String,
    pub file_id: String,
    pub upload_date: i64,
}