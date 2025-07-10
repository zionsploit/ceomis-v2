use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use services::request::settings::RequestBarangayType;

#[derive(DeriveEntity, Default, Debug, Clone, Copy)]
#[sea_orm(table_name = "settings_barangay")]
pub struct Entity;

#[derive(DeriveEntityModel, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub barangay_type: Option<BarangayType>,
    pub is_poblacion: bool
}

#[derive(DeriveRelation, Clone, EnumIter, Debug)]
pub enum Relation {}

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "barangay_type")]
pub enum BarangayType {
    #[sea_orm(string_value = "Rural")]
    Rural,
    #[sea_orm(string_value = "Urban")]
    Urban
}

impl From<RequestBarangayType> for BarangayType {
    fn from(value: RequestBarangayType) -> Self {
        match value {
            RequestBarangayType::Rural => BarangayType::Rural,
            RequestBarangayType::Urban => BarangayType::Urban
        }
    }
}

impl ActiveModelBehavior for ActiveModel  {}