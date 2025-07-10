use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(DeriveEntity, Debug, Default, Clone, Copy)]
#[sea_orm(table_name = "projects_monitoring_img")]
pub struct Entity;

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, DeriveEntityModel, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub remarks_id: i32,
    pub images_key: String,
    pub images_original_name: String
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {
    ProjectsMonitoringRemarks
}

impl RelationTrait for Relation  {
    fn def(&self) -> RelationDef {
        match self {
            Self::ProjectsMonitoringRemarks => Entity::belongs_to(super::projects_monitoring_remarks::Entity)
                .from(Column::RemarksId)
                .to(super::projects_monitoring_remarks::Column::Id)
                .into(),
        }
    }
}

impl Related<super::projects_monitoring_remarks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectsMonitoringRemarks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}