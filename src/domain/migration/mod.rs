use sea_orm_migration::prelude::*;
use std::vec::Vec;

mod user_migration;
mod user_preferences_migration;
mod profile_photo_migration;
mod verification_migration;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(user_migration::Migration),
            Box::new(user_preferences_migration::Migration),
            Box::new(profile_photo_migration::Migration),
            Box::new(verification_migration::Migration),
        ]
    }
}