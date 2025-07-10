use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, DeriveEntity)]
#[sea_orm(table_name = "projects")]
pub struct Entity;

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, DeriveEntityModel, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub project_year: i32,
    pub project_name: String,
    pub project_code: String,
    pub project_status: Option<ProjectStatus>, 
    pub barangays: Option<Vec<i32>>,
    pub appropriation: Option<i32>,
    pub approved_budget_contact: Option<i32>,
    pub contractor_id: Option<i32>,
    pub contract_cost: Option<i32>,
    pub start_date: Option<String>,
    pub calendar_days: Option<String>,
    pub time_extensions: Option<i32>,
    pub target_date: Option<String>,
    pub project_type_id: Option<i32>,
    pub project_category_id: Option<i32>,
    pub project_sof_id: Option<i32>,
    pub project_incharge_id: Option<i32>,
    pub sustainable_development_goals: Option<Vec<i32>>,
    pub sector: Option<Vec<i32>>,
    pub project_takers_id: Option<i32>,
    pub accomplished: Option<i16>,
    pub remarks: Option<String>,
    pub prepared_users_id: Option<i32>,
    pub is_disposed: Option<bool>
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {
    Contractors,
    SettingsType,
    SettingsCategories,
    SettingsSof,
    SettingsIncharge,
    SettingsTakers,
    UserInfo,
    ProjectsInfraCode,
    ProjectsPayment,
}

impl RelationTrait for Relation  {
    fn def(&self) -> RelationDef {
        match self {
            Self::ProjectsPayment => Entity::has_many(super::projects_payment::Entity).into(),
            Self::ProjectsInfraCode => Entity::has_one(super::projects_infra_code::Entity).into(),
            Self::Contractors => Entity::belongs_to(super::contractors::Entity)
                .from(Column::ContractorId)
                .to(super::contractors::Column::Id)
                .into(),
            Self::SettingsType => Entity::belongs_to(super::settings::settings_type::Entity)
                .from(Column::ProjectTypeId)
                .to(super::settings::settings_type::Column::Id)
                .into(),
            Self::SettingsCategories => Entity::belongs_to(super::settings::settings_categories::Entity)
                .from(Column::ProjectCategoryId)
                .to(super::settings::settings_categories::Column::Id)
                .into(),
            Self::SettingsSof => Entity::belongs_to(super::settings::settings_sof::Entity)
                .from(Column::ProjectSofId)
                .to(super::settings::settings_sof::Column::Id)
                .into(),
            Self::SettingsIncharge => Entity::belongs_to(super::settings::settings_incharge::Entity)
                .from(Column::ProjectInchargeId)
                .to(super::settings::settings_incharge::Column::Id)
                .into(),
            Self::SettingsTakers => Entity::belongs_to(super::settings::settings_takers::Entity)
                .from(Column::ProjectTakersId)
                .to(super::settings::settings_takers::Column::Id)
                .into(),
            Self::UserInfo => Entity::belongs_to(super::user_info::Entity)
                .from(Column::PreparedUsersId)
                .to(super::user_info::Column::Id)
                .into(),
        }
    }
}

impl Related<super::projects_payment::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::ProjectsPayment.def()
    }
}

impl Related<super::projects_infra_code::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::ProjectsInfraCode.def()
    }
}

impl Related<super::contractors::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::Contractors.def()
    }
}

impl Related<super::settings::settings_type::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::SettingsType.def()
    }
}

impl Related<super::settings::settings_type::Entity> for Relation  {
    fn to() -> RelationDef {
        Relation::SettingsType.def()
    }
}

impl Related<super::settings::settings_categories::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::SettingsCategories.def()
    }
}

impl Related<super::settings::settings_sof::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::SettingsSof.def()
    }
}

impl Related<super::settings::settings_incharge::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::SettingsIncharge.def()
    }
}

impl Related<super::settings::settings_takers::Entity> for Entity  {
    fn to() -> RelationDef {
        Relation::SettingsTakers.def()
    }
}

impl Related<super::user_info::Entity> for Relation  {
    fn to() -> RelationDef {
        Relation::UserInfo.def()
    }
}

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "project_status")]
pub enum ProjectStatus {
    #[sea_orm(string_value = "Not Yet Started")]
    NotYetStarted,
    #[sea_orm(string_value = "Preparation")]
    Preparation,
    #[sea_orm(string_value = "Bidding")]
    Bidding,
    #[sea_orm(string_value = "Bidded")]
    Bidded,
    #[sea_orm(string_value = "On-Going")]
    OnGoing,
    #[sea_orm(string_value = "Completed")]
    Completed,
    #[sea_orm(string_value = "Suspended")]
    Suspended
}

impl ActiveModelBehavior for ActiveModel {}