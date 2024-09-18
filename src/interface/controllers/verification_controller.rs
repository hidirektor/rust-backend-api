use crate::domain::services::verification_service::VerificationService;
use crate::interface::dto::verification_dto::{SendOtpRequest, VerifyOtpRequest};
use actix_web::{web, HttpResponse};
use std::sync::Arc;

#[derive(Clone)]
pub struct VerificationController {
    pub verification_service: Arc<VerificationService>,
}

impl VerificationController {
    pub fn new(verification_service: Arc<VerificationService>) -> Self {
        Self { verification_service }
    }

    pub async fn send_otp(
        verification_service: web::Data<VerificationService>,
        send_otp_req: web::Json<SendOtpRequest>,
    ) -> HttpResponse {
        // OTP gönderme işlemleri
        HttpResponse::Ok().body("OTP sent successfully")
    }

    pub async fn verify_otp(
        verification_service: web::Data<VerificationService>,
        verify_otp_req: web::Json<VerifyOtpRequest>,
    ) -> HttpResponse {
        // OTP doğrulama işlemleri
        HttpResponse::Ok().body("OTP verified successfully")
    }
}