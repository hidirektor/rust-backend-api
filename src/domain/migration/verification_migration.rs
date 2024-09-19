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
                    .table(Verification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Verification::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Verification::UserId).string().not_null()) // user_id alanı, string ve not_null
                    .col(ColumnDef::new(Verification::MailVerified).boolean().not_null()) // mail_verified boolean ve not_null
                    .col(ColumnDef::new(Verification::PhoneVerified).boolean().not_null()) // phone_verified boolean ve not_null
                    .col(ColumnDef::new(Verification::MailVerificationDate).big_integer().not_null()) // mail_verification_date big_integer ve not_null
                    .col(ColumnDef::new(Verification::PhoneVerificationDate).big_integer().not_null()) // phone_verification_date big_integer ve not_null
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Verification::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Verification {
    Table,
    Id,
    UserId,
    MailVerified,
    PhoneVerified,
    MailVerificationDate,
    PhoneVerificationDate,
}