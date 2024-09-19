mod domain;
mod infrastructure;
mod interface;

use actix_web::{web, App, HttpServer};
use domain::migration::Migrator;
use infrastructure::config::DBConfig;
use infrastructure::{cache, database, mailer, messaging};
use interface::routes::configure;
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Yapılandırmaları yükle
    let config = DBConfig::from_env();

    // Veritabanı bağlantısı
    let db = database::connect(&config.database_url).await;

    // Migrasyonları çalıştır
    if let Err(err) = Migrator::up(&db, None).await {
        eprintln!("Migration failed: {:?}", err);
    }

    // Redis bağlantısı
    let redis_client = cache::redis_client::connect(&config.redis_url);

    // RabbitMQ bağlantısı
    let rabbitmq_connection = messaging::rabbitmq_client::connect(&config.rabbitmq_url).await;

    // MinIO bağlantısı
    /*let minio_client = storage::minio_client::connect(
        &config.minio_endpoint,
        &config.minio_access_key,
        &config.minio_secret_key,
    );*/

    // Mailer (SMTP) istemcisi
    let mail_client = mailer::mail_client::MailClient::new(
        &config.smtp_host,
        config.smtp_port,
        &config.smtp_user,
        &config.smtp_pass,
        &config.smtp_user,
    );

    // Repositories oluşturun
    let user_repository = Arc::new(
        infrastructure::database::repositories_impl::user_repository_impl::UserRepositoryImpl::new(db.clone())
    );
    let user_preferences_repository = Arc::new(
        infrastructure::database::repositories_impl::user_preferences_repository_impl::UserPreferencesRepositoryImpl::new(db.clone())
    );
    let profile_photo_repository = Arc::new(
        infrastructure::database::repositories_impl::profile_photo_repository_impl::ProfilePhotoRepositoryImpl::new(db.clone())
    );
    let verification_repository = Arc::new(
        infrastructure::database::repositories_impl::verification_repository_impl::VerificationRepositoryImpl::new(db.clone())
    );

    // Servisleri oluşturun
    let auth_service = Arc::new(domain::services::auth_service::AuthService::new(
        user_repository.clone(),
        redis_client.clone(),
        config.jwt_secret.clone(),
    ));

    let user_service = Arc::new(domain::services::user_service::UserService::new(
        user_repository.clone(),
        user_preferences_repository.clone(),
        redis_client.clone(),
    ));

    let verification_service = Arc::new(domain::services::verification_service::VerificationService::new(
        user_repository.clone(),
        verification_repository.clone(),
        redis_client.clone(),
    ));

    // Controller'ları oluşturun
    let auth_controller = interface::controllers::auth_controller::AuthController::new(auth_service.clone());
    let user_controller = interface::controllers::user_controller::UserController::new(user_service.clone());
    let verification_controller = interface::controllers::verification_controller::VerificationController::new(verification_service.clone());

    // HTTP Sunucusunu başlatın
    let bind_address = format!("0.0.0.0:{}", config.api_port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(verification_service.clone()))
            .configure(|cfg| {
                configure(
                    cfg,
                    auth_controller.clone(),
                    user_controller.clone(),
                    verification_controller.clone(),
                );
            })
    })
        .bind(bind_address)?
        .run()
        .await
}