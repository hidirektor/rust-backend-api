use sea_orm::DeriveIden;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserPreferences::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserPreferences::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(UserPreferences::UserId).string().not_null())
                    .col(ColumnDef::new(UserPreferences::Language).boolean().not_null().default(false))
                    .col(ColumnDef::new(UserPreferences::NightMode).boolean().not_null().default(false))
                    .col(ColumnDef::new(UserPreferences::MailNotification).boolean().not_null().default(false))
                    .col(ColumnDef::new(UserPreferences::SmsNotification).boolean().not_null().default(false))
                    .col(ColumnDef::new(UserPreferences::PushNotification).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(UserPreferences::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum UserPreferences {
    Table,
    Id,
    UserId,
    Language,
    NightMode,
    MailNotification,
    SmsNotification,
    PushNotification,
}