use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub user_name: String,
    pub name_surname: String,
    pub email: String,
    pub phone_number: String,
    pub company_name: String,
    pub password: String,
    pub is_active: bool,
}