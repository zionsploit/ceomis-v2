use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::projects;

#[derive(DeriveEntity, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[sea_orm(table_name = "settings_incharge")]
pub struct Entity;

#[derive(DeriveEntityModel, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String
}

#[derive(Debug, EnumIter)]
pub enum Relation {
    Projects
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Projects => Entity::has_many(projects::Entity).into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel  {}