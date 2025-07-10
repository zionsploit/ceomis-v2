use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::projects;

#[derive(DeriveEntity, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[sea_orm(table_name = "settings_takers")]
pub struct Entity;

#[derive(DeriveEntityModel, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub contact_number: Option<String>
}

#[derive(Debug, EnumIter)]
pub enum Relation {
    Projects
}

impl RelationTrait for Relation  {
    fn def(&self) -> RelationDef {
        match self {
            Self::Projects => Entity::has_many(projects::Entity).into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel  {}