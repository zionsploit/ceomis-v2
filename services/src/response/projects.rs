use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::response::settings::{ResponseBarangays, ResponseSector, ResponseSustainableDevelopmentGoals};


#[derive(Serialize, Default, TS, Deserialize, FromQueryResult)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectsOverview {
    pub total_projects: Option<i64>,
    pub total_unimplemented: Option<i64>,
    pub total_preparing: Option<i64>,
    pub total_bidded: Option<i64>,
    pub total_bidding: Option<i64>,
    pub total_ongoing: Option<i64>,
    pub total_completed: Option<i64>,
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult, Debug)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectsStatsTypes {
    pub name: String,
    pub projects_total: i64,
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectsStatsCategory {
    pub name: String,
    pub projects_total: i64,
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectsSectorWithAppropriation {
    pub name: String,
    pub value: i32
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectsTop10Appropriation {
    pub id: i32,
    pub name: String,
    pub project_code: String,
    pub appropriation: i32,
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectsTop10Awared {
    pub constructor_id: i32,
    pub name: String,
    pub total_projects: i64,
    pub total_appropriation: Option<i64>,
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectsByFund {
   pub projects_id: i32,
   pub projects_name: String,
   pub project_code: String,
   pub project_status: Option<String>,
   pub contract_cost: Option<i32>,
   pub contractor_id: Option<i32>,
   pub contractor_name: Option<String>
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectForContractors {
    pub project_id: i32,
    pub project_name: String,
    pub project_code: String,
    pub project_status: Option<String>,
    pub project_takers: Option<String>,
    pub project_appropriation: i32,
    pub project_contract_cost: i32,
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectsById {
   pub projects_id: i32,
   pub projects_name: String,
   pub projects_code: String,
   pub project_year: i32,
   pub projects_status: Option<String>,
   pub projects_appropriation: Option<i32>,
   pub projects_approved_budget_contract: Option<i32>,
   pub projects_accomplished: Option<i16>,
   pub projects_remarks: Option<String>,
   pub projects_contract_cost: Option<i32>,
   pub projects_start_date: Option<String>,
   pub projects_target_date: Option<String>,
   pub projects_barangay: Option<Vec<ResponseBarangays>>,
   pub projects_sdg: Option<Vec<ResponseSustainableDevelopmentGoals>>,
   pub projects_sector: Option<Vec<ResponseSector>>,
   pub s_types_name: Option<String>,
   pub s_category_name: Option<String>,
   pub s_sof_name: Option<String>,
   pub s_incharge_name: Option<String>,
   pub s_takers_name: Option<String>,
   pub contractor_name: Option<String>,
   pub infra_code: Option<String>
}

#[derive(Serialize, Default, TS, Deserialize, Clone)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectRemarksImg {
   pub id: i32,
   pub remarks_id: i32,
   pub images_key: String,
   pub images_original_name: String 
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectRemarks {
   pub id: i32,
   pub remarks: String,
   pub remarks_date: String,
   pub project_id: i32,
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseViewProjectsById {
   pub projects_info: ResponseProjectsById,
   pub projects_remarks_image: Vec<ResponseProjectRemarksImg>,
   pub projects_remarks: Vec<ResponseProjectRemarks>,
   pub projects_payment: Vec<ResponseProjectPayment>
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Settings.ts")]
pub struct ResponseProjectInfraCode {
   pub id: i32,
   pub project_id: i32,
   pub project_code: String
}

#[derive(Serialize, Deserialize, TS, FromQueryResult, Clone)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub struct ResponseProjectPayment {
    pub id: i32,
    pub project_id: i32,
    pub billing_date: String,
    pub processed_by: String,
    pub amount_due: String,
    pub amount_paid: i32,
    pub reference_no: String,
    pub payment_date: String
}