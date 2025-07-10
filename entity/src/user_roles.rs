use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, DeriveEntity)]
#[sea_orm(table_name = "user_roles")]
pub struct Entity;

#[derive(Clone, Debug, DeriveEntityModel, Deserialize, Serialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    name: String
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum UserRolesName {
    #[sea_orm(string_value = "Bac Office")]
    BacOffice,
    #[sea_orm(string_value = "Engineer's Office")]
    EngineersOffice,
    #[sea_orm(string_value = "Mayor's Office")]
    MayorsOffice,
    #[sea_orm(string_value = "Planning Office")]
    PlanningOffce,
    #[sea_orm(string_value = "System Admin")]
    SystemAdmin
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::user::Entity")]
    User
}

impl Related<super::user::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel  {}