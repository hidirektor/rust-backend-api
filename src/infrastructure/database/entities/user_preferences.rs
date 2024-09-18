use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "UserPreferences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: String,
    pub language: bool,
    pub night_mode: bool,
    pub mail_notification: bool,
    pub sms_notification: bool,
    pub push_notification: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined") // Empty relation definition
    }
}

impl ActiveModelBehavior for ActiveModel {}