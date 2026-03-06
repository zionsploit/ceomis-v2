use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, DeriveEntity)]
#[sea_orm(table_name = "user_info")]
pub struct Entity;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    first_name: String,
    middle_name: String,
    last_name: String,
    user_id: i32
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User
}

impl RelationTrait for Relation  {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .into()
        }
    }
}

impl Related<super::user::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}