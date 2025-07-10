use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::projects;

#[derive(DeriveEntity, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[sea_orm(table_name = "settings_type")]
pub struct Entity;

#[derive(DeriveEntityModel, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Debug, EnumIter)]
pub enum Relation {
    Projects,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Projects => Entity::has_many(projects::Entity).into()
        }
    }
}

impl Related<crate::projects::Entity> for Relation  {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}