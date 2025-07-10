use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(DeriveEntity, Debug, Default, Clone, Copy)]
#[sea_orm(table_name = "projects_monitoring_remarks")]
pub struct Entity;

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, DeriveEntityModel, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub remarks: String,
    pub remarks_date: String,
    pub project_id: i32
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {
    Projects,
    ProjectsMonitoringImg
}

impl sea_orm::RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        match self {
            Self::Projects => Entity::belongs_to(super::projects::Entity)
                .from(Column::ProjectId)
                .to(super::projects::Column::Id)
                .into(),
            Self::ProjectsMonitoringImg => Entity::has_many(super::projects_monitoring_img::Entity).into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}