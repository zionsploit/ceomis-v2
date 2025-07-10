use serde::Deserialize;
use ts_rs::TS;

use crate::helpers::Helpers;

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub enum ProjectStatus {
    NotYetStarted,
    Preparation,
    Bidding,
    Bidded,
    OnGoing,
    Completed,
    Suspended
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub struct RequestAddProjects {
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
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub struct RequestUpdateProjects {
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
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub struct RequestDisposedProjectsById {
    pub id: i32,
    pub user_password: String
}

impl RequestDisposedProjectsById {
    pub fn hash_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub struct RequestUpsertProjectsInfraCode {
    pub id: Option<i32>,
    pub projects_id: i32,
    pub projects_code: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub struct RequestDeleteProjectRemarks {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteProjectRemarks {
    pub fn hash_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}

// PROJECT PAYMENT

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub struct RequestAddProjectPayment {
    pub project_id: i32,
    pub billing_date: String,
    pub processed_by: String,
    pub amount_due: String,
    pub amount_paid: i32,
    pub reference_no: String,
    pub payment_date: String
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub struct RequestDeleteProjectsPayment {
    pub id: i32,
    pub user_password: String
}

impl RequestDeleteProjectsPayment {
    pub fn hash_password(&self) -> String {
        Helpers::string_to_sha256(&self.user_password)
    }
}


#[derive(Deserialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Projects.ts")]
pub struct RequestGenerateProjectsProfile {
    pub id: i32,
}