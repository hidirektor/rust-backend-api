use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Verification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: String,
    pub mail_verified: bool,
    pub phone_verified: bool,
    pub mail_verification_date: i64,
    pub phone_verification_date: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined") // Empty relation definition
    }
}

impl ActiveModelBehavior for ActiveModel {}