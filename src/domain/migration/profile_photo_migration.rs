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
                    .table(ProfilePhoto::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProfilePhoto::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(ProfilePhoto::UserId).string().not_null())
                    .col(ColumnDef::new(ProfilePhoto::UserName).string().not_null())
                    .col(ColumnDef::new(ProfilePhoto::FileId).string().not_null())
                    .col(ColumnDef::new(ProfilePhoto::UploadDate).big_integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ProfilePhoto::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum ProfilePhoto {
    Table,
    Id,
    UserId,
    UserName,
    FileId,
    UploadDate,
}