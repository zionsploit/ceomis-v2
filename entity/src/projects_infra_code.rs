use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(DeriveEntity, Debug, Default, Clone, Copy)]
#[sea_orm(table_name = "projects_infra_code")]
pub struct Entity;

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, DeriveEntityModel, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub project_id: i32,
    pub project_code: String
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {
    Projects
}

impl RelationTrait for Relation  {
    fn def(&self) -> RelationDef {
        match self {
            Self::Projects => Entity::belongs_to(super::projects::Entity)
                .from(Column::ProjectId)
                .to(super::projects::Column::Id)
                .into()
        }
    }
}

impl Related<super::projects::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl Related<super::projects::Entity> for Relation  {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}