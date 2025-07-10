use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::response::projects::ResponseProjectForContractors;

#[derive(Serialize, Deserialize, TS, Default)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseContractors {
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

#[derive(Serialize, Deserialize, TS, Default)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseContractorsWithProjects {
    pub contractor: ResponseContractors,
    pub projects: Vec<ResponseProjectForContractors>
}