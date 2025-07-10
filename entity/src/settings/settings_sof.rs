use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::projects;

#[derive(Clone, Copy, Debug, Default, DeriveEntity, PartialEq, Eq, PartialOrd, Ord)]
#[sea_orm(table_name = "settings_sof")]
pub struct Entity;

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, DeriveEntityModel)]
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