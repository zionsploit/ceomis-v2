use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, DeriveEntity)]
#[sea_orm(table_name = "contractors")]
pub struct Entity;

#[derive(Clone, Debug, Default, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email_address: String,
    pub address_street: String,
    pub address_barangay: String,
    pub address_municipality: String,
    pub address_province: String,
    pub about: Option<String>,
    pub contact_full_name: String,
    pub contact_position: String,
    pub contact_number: String,
    pub is_delete: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Projects
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Projects => Entity::has_many(super::projects::Entity).into()
        }
    }
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}