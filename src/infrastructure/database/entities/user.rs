use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: String,
    pub user_name: String,
    pub name_surname: String,
    pub email: String,
    pub phone_number: String,
    pub company_name: String,
    pub password: String,
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations are defined for this entity") // If there are no relations
    }
}

impl ActiveModelBehavior for ActiveModel {}