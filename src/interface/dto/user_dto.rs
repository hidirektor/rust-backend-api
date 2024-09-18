use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct GetProfileRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct GetProfileResponse {
    pub user_id: String,
    pub user_name: String,
    pub name_surname: String,
    pub email: String,
    pub phone_number: String,
    pub company_name: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub user_id: String,
    pub user_data: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub user_name: String,
    pub old_password: String,
    pub new_password: String,
    pub close_sessions: bool,
}