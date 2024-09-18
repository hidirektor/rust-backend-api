use dotenv::dotenv;
use std::env;

pub struct DBConfig {
    pub database_url: String,
    pub redis_url: String,
    pub rabbitmq_url: String,
    pub minio_endpoint: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub jwt_secret: String,
    pub bucket_name: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub onesignal_rest_api: String,
    pub api_port: String,
}

impl DBConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        // Veritabanı URL'si oluşturma
        let db_host = env::var("DB_HOST").expect("DB_HOST not set");
        let db_port = env::var("DB_PORT").expect("DB_PORT not set");
        let db_name = env::var("DB_NAME").expect("DB_NAME not set");
        let db_user = env::var("DB_USER").expect("DB_USER not set");
        let db_pass = env::var("DB_PASS").expect("DB_PASS not set");
        let database_url = format!("mysql://{}:{}@{}:{}/{}", db_user, db_pass, db_host, db_port, db_name);

        // Redis URL'si oluşturma
        let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST not set");
        let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT not set");
        let redis_username = env::var("REDIS_USERNAME").unwrap_or_default();
        let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();
        let redis_url = format!("redis://{}:{}@{}:{}/", redis_username, redis_password, redis_host, redis_port);

        // RabbitMQ URL'si oluşturma
        let rabbitmq_host = env::var("RABBITMQ_HOST").expect("RABBITMQ_HOST not set");
        let rabbitmq_port = env::var("RABBITMQ_PORT").expect("RABBITMQ_PORT not set");
        let rabbitmq_username = env::var("RABBITMQ_USERNAME").expect("RABBITMQ_USERNAME not set");
        let rabbitmq_password = env::var("RABBITMQ_PASSWORD").expect("RABBITMQ_PASSWORD not set");
        let rabbitmq_url = format!("amqp://{}:{}@{}:{}/%2f", rabbitmq_username, rabbitmq_password, rabbitmq_host, rabbitmq_port);

        // MinIO Endpoint
        let minio_endpoint = env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT not set");
        let minio_port = env::var("MINIO_PORT").expect("MINIO_PORT not set");
        let minio_access_key = env::var("MINIO_ACCESS_KEY").expect("MINIO_ACCESS_KEY not set");
        let minio_secret_key = env::var("MINIO_SECRET_KEY").expect("MINIO_SECRET_KEY not set");
        let minio_endpoint = format!("http://{}:{}", minio_endpoint, minio_port);

        // SMTP Ayarları
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST not set");
        let smtp_port = env::var("SMTP_PORT").expect("SMTP_PORT not set").parse::<u16>().expect("Invalid SMTP_PORT");
        let smtp_user = env::var("SMTP_USER").expect("SMTP_USER not set");
        let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS not set");

        // Diğer Ayarlar
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
        let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not set");
        let onesignal_rest_api = env::var("ONESIGNAL_REST_API").unwrap_or_default();
        let api_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

        Self {
            database_url,
            redis_url,
            rabbitmq_url,
            minio_endpoint,
            minio_access_key,
            minio_secret_key,
            jwt_secret,
            bucket_name,
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_pass,
            onesignal_rest_api,
            api_port,
        }
    }
}