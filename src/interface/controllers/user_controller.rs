use crate::domain::services::user_service::UserService;
use crate::interface::dto::user_dto::{ChangePasswordRequest, GetProfileRequest, GetProfileResponse, UpdateProfileRequest};
use actix_web::{web, HttpResponse};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserController {
    pub user_service: Arc<UserService>,
}

impl UserController {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    pub async fn get_profile(
        user_service: web::Data<UserService>,
        profile_req: web::Json<GetProfileRequest>,
    ) -> HttpResponse {
        match user_service.get_profile(&profile_req.user_id).await {
            Ok(Some(user)) => HttpResponse::Ok().json(GetProfileResponse {
                user_id: user.user_id,
                user_name: user.user_name,
                name_surname: user.name_surname,
                email: user.email,
                phone_number: user.phone_number,
                company_name: user.company_name,
            }),
            Ok(None) => HttpResponse::NotFound().body("User not found"),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }

    pub async fn update_profile(
        user_service: web::Data<UserService>,
        update_req: web::Json<UpdateProfileRequest>,
    ) -> HttpResponse {
        // Güncelleme işlemleri
        HttpResponse::Ok().body("Profile updated successfully")
    }

    pub async fn change_password(
        user_service: web::Data<UserService>,
        change_pass_req: web::Json<ChangePasswordRequest>,
    ) -> HttpResponse {
        match user_service.change_password(
            &change_pass_req.user_name,
            &change_pass_req.old_password,
            &change_pass_req.new_password,
            change_pass_req.close_sessions,
        ).await {
            Ok(_) => HttpResponse::Ok().body("Password changed successfully"),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }
}