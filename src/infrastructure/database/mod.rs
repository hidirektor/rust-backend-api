pub mod entities;
pub mod repositories_impl;

use sea_orm::{Database, DatabaseConnection};

pub async fn connect(database_url: &str) -> DatabaseConnection {
    Database::connect(database_url)
        .await
        .expect("Failed to connect to the database")
}