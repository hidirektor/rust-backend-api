use crate::domain::services::auth_service::AuthService;
use crate::interface::dto::auth_dto::{LoginRequest, LoginResponse, LogoutRequest, RegisterRequest};
use actix_web::{web, HttpResponse};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthController {
    pub auth_service: Arc<AuthService>,
}

impl AuthController {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn login(
        auth_service: web::Data<AuthService>,
        login_req: web::Json<LoginRequest>,
    ) -> HttpResponse {
        match auth_service.login(&login_req.user_name, &login_req.password).await {
            Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
            Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
        }
    }

    pub async fn register(
        auth_service: web::Data<AuthService>,
        register_req: web::Json<RegisterRequest>,
    ) -> HttpResponse {
        let user = crate::domain::entities::user::User {
            id: 0,
            user_id: String::new(),
            user_name: register_req.user_name.clone(),
            name_surname: register_req.name_surname.clone(),
            email: register_req.email.clone(),
            phone_number: register_req.phone_number.clone(),
            company_name: register_req.company_name.clone(),
            password: register_req.password.clone(),
            is_active: true,
        };
        match auth_service.register(user).await {
            Ok(_) => HttpResponse::Ok().body("User registered successfully"),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }

    pub async fn logout(
        auth_service: web::Data<AuthService>,
        logout_req: web::Json<LogoutRequest>,
    ) -> HttpResponse {
        match auth_service.logout(&logout_req.token).await {
            Ok(_) => HttpResponse::Ok().body("Logged out successfully"),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }
}