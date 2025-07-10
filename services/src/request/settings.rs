use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::helpers::Helpers;

// Sustainable Development Goals
#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestGetSustainableDevelopmentGoalsById {
    pub id: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestAddSustainableDevelopmentGoals {
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestUpdateSustainableDevelopmentGoals {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestDeleteSustainableDevelopmentGoals {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteSustainableDevelopmentGoals {
    pub fn hash_request_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}

// Source Of Funds
#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestAddSourceOfFunds {
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestUpdateSourceOfFunds {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestDeleteSourceOfFunds {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteSourceOfFunds {
     pub fn hash_request_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}

// Types
#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestAddTypes {
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestUpdateTypes {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestDeleteTypes {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteTypes {
    pub fn hash_request_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}

// Incharge
#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestAddIncharge {
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestUpdateIncharge {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestDeleteIncharge {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteIncharge {
    pub fn hash_request_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}

// Categories
#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestAddCategories {
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestUpdateCategories {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestDeleteCategories {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteCategories {
    pub fn hash_request_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}

// Sector
#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestAddSector {
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestUpdateSector {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestDeleteSector {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteSector {
    pub fn hash_request_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}

// Baragay
#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub enum RequestBarangayType {
    Rural,
    Urban,
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestAddBarangay {
    pub name: String,
    pub barangay_type: Option<RequestBarangayType>,
    pub is_poblacion: bool
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestUpdateBarangay {
    pub id: i32,
    pub name: String,
    pub barangay_type: Option<RequestBarangayType>,
    pub is_poblacion: bool
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestDeleteBarangay {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteBarangay {
    pub fn hash_request_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}

// Takers
#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestAddTakers {
    pub name: String,
    pub contact_number: Option<String>,
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestUpdateTakers {
    pub id: i32,
    pub name: String,
    pub contact_number: Option<String>,
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct RequestDeleteTakers {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteTakers {
    pub fn hash_request_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}