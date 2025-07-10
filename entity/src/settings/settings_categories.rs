use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::projects;

#[derive(DeriveEntity, Default, Debug, Clone, Copy)]
#[sea_orm(table_name = "settings_categories")]
pub struct Entity;

#[derive(DeriveEntityModel, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String
}

#[derive(EnumIter, Debug)]
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

impl ActiveModelBehavior for ActiveModel {}