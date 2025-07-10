use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, DeriveEntity, PartialEq, Eq, PartialOrd, Ord)]
#[sea_orm(table_name = "settings_sdg")]
pub struct Entity;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, DeriveEntityModel, Debug, Default, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel  {}