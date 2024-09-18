use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub user_name: String,
    pub name_surname: String,
    pub email: String,
    pub phone_number: String,
    pub company_name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub token: String,
}