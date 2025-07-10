use serde::Deserialize;
use ts_rs::TS;

use crate::helpers::Helpers;


#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestAddContractors {
    pub name: String,
    pub email_address: String,
    pub address_street: String,
    pub address_barangay: String,
    pub address_municipality: String,
    pub address_province: String,
    pub about: Option<String>,
    pub contact_full_name: String,
    pub contact_position: String,
    pub contact_number: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestUpdateContractors {
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
    pub contact_number: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestDeleteContractorsById {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteContractorsById {
    pub fn hash_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}


#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestGenerateProjectsContractors {
    pub id: i32,
}