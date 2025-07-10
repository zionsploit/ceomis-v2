use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(DeriveEntity, Default, Debug, Clone, Copy)]
#[sea_orm(table_name = "settings_sector")]
pub struct Entity;

#[derive(DeriveEntityModel, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String
}

#[derive(DeriveRelation, Clone, EnumIter, Debug)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel  {}