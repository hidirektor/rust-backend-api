use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendOtpRequest {
    pub user_name: String,
    pub otp_type: String,
}

#[derive(Deserialize)]
pub struct VerifyOtpRequest {
    pub user_name: String,
    pub otp_code: String,
}

#[derive(Deserialize)]
pub struct SendVerificationMailRequest {
    pub user_name: String,
}

#[derive(Deserialize)]
pub struct VerifyLinkRequest {
    pub user_id: String,
    pub verification_code: String,
}