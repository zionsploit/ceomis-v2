use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{request::settings::RequestBarangayType, response::contractors::ResponseContractors};

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseSustainableDevelopmentGoals {
    pub id: i32,
    pub name: String
}


#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseSourceOfFunds {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseTypes {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseIncharge {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseCategories {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseSector {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseBarangays {
    pub id: i32,
    pub name: String,
    pub barangay_type: Option<RequestBarangayType>,
    pub is_poblacion: bool
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseTakers {
    pub id: i32,
    pub name: String,
    pub contact_number: Option<String>,
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponsePrepareAllSettings {
    pub s_barangay: Vec<ResponseBarangays>,
    pub contractors: Vec<ResponseContractors>,
    pub s_type: Vec<ResponseTypes>,
    pub s_categories: Vec<ResponseCategories>,
    pub s_sof: Vec<ResponseSourceOfFunds>,
    pub s_incharge: Vec<ResponseIncharge>,
    pub s_sdg: Vec<ResponseSustainableDevelopmentGoals>,
    pub s_sector: Vec<ResponseSector>,
    pub s_takers: Vec<ResponseTakers> 
}