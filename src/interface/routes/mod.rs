use crate::interface::controllers::{
    auth_controller::AuthController,
    user_controller::UserController,
    verification_controller::VerificationController,
};
use actix_web::web;

pub fn configure(
    cfg: &mut web::ServiceConfig,
    auth_controller: AuthController,
    user_controller: UserController,
    verification_controller: VerificationController,
) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(AuthController::login))
            .route("/register", web::post().to(AuthController::register))
            .route("/logout", web::post().to(AuthController::logout)),
    );

    cfg.service(
        web::scope("/user")
            .route("/get-profile", web::post().to(UserController::get_profile))
            .route("/update-profile", web::post().to(UserController::update_profile))
            .route("/change-password", web::post().to(UserController::change_password)),
    );

    cfg.service(
        web::scope("/verification")
            .route("/send-otp", web::post().to(VerificationController::send_otp))
            .route("/verify-otp", web::post().to(VerificationController::verify_otp)),
    );
}