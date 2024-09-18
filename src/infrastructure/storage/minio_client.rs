use minio_rsc::provider::StaticProvider;
use minio_rsc::Minio;
use std::sync::Arc;

pub fn connect(endpoint: &str, access_key: &str, secret_key: &str) -> Arc<Minio> {
    let provider = StaticProvider::new(access_key, secret_key, None);
    let minio = Minio::builder()
        .endpoint(endpoint)
        .provider(provider)
        .secure(false) // HTTPS off
        .build()
        .expect("Failed to create Minio client");
    Arc::new(minio)
}