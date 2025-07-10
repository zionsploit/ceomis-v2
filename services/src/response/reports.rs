
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryListOfProjects {
    pub project_id: i32,
    pub project_name: String,
    pub project_code: String,
    pub project_appropriation: Option<i32>,
    pub project_bid_date: Option<String>,
    pub project_contract_cost: Option<i32>,
    pub contractor_id: Option<i32>,
    pub contractor_name: Option<String>,
    pub sof_id: Option<i32>,
    pub sof_name: Option<String>,
    pub project_status: Option<String>,
    pub project_accomplished: Option<i16>,
    pub project_taker_id: Option<i32>,
    pub project_taker_name: Option<String>,
    pub project_year: i32,
    pub project_remarks: Option<String>
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryListOfProjectsOverview {
    pub summary_list_of_reports: Vec<ResponseSummaryListOfProjects>,
    pub total_records: usize,
    pub total_appropriation: i64,
    pub total_contract_cost: i64
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult, Clone, Debug)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryImplementationByYear {
    pub project_year: i32,
    pub project_status: String,
    pub total_projects: i64,
    pub total_appropriation: i64,
}

#[derive(Serialize, Default, TS, Deserialize, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryImplementationByYearOverview {
    pub year: i32,
    pub data: Vec<ResponseSummaryImplementationByYear>
}

#[derive(Serialize, Default, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryImplementationByYearFullOverview {
    pub projects_data: Vec<ResponseSummaryImplementationByYearOverview>,
    pub total_status: i32,
    pub total_projects: i64,
    pub total_appropriation: i64,
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryProjectsPerType {
    pub project_type: String,
    pub total_projects: i64,
    pub total_appropriation: i64
}

#[derive(Serialize, Default, TS, Deserialize, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryProjectsPerTypeOverview {
    pub total_types: usize,
    pub total_projects: i64,
    pub total_appropriation: i64,
    pub data: Vec<ResponseSummaryProjectsPerType>
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryProjectSavingsReport {
    pub project_name: String,
    pub project_code: String,
    pub project_status: Option<String>,
    pub contractor_name: Option<String>,
    pub taker_name: Option<String>,
    pub approved_budget_contract: Option<i32>,
    pub appropriation: Option<i32>,
    pub contract_cost: Option<i32>,
    pub savings: i32,
}

#[derive(Serialize, Default, TS, Deserialize, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryProjectSavingsReportOverview {
    pub data: Vec<ResponseSummaryProjectSavingsReport>,
    pub total_records: usize,
    pub total_appropriation: i64,
    pub total_abc: i64,
    pub total_contract_cost: i64,
    pub total_savings: i64
}


#[derive(Serialize, Default, TS, Deserialize, FromQueryResult, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummarySlippageReport {
    pub project_name: String,
    pub contractor_name: String,
    pub taker_name: String,
    pub start_date: String,
    pub target_date: String,
    pub days_lapse: i32,
    pub remarks: String,
}

#[derive(Serialize, Default, TS, Deserialize, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummarySlippageReportOverview {
    pub data: Vec<ResponseSummarySlippageReport>,
    pub total_records: usize
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryFinancialStatusProjects {
    pub project_name: String,
    pub project_code: String,
    pub contractor_name: Option<String>,
    pub taker_name: Option<String>,
    pub project_status: Option<String>,
    pub project_start_date: Option<String>,
    pub project_target_date: Option<String>,
    pub project_contract_cost: Option<i32>,
    pub project_paid: Option<i64>
}

#[derive(Serialize, Default, TS, Deserialize, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryFinancialStatusProjectsOverview {
    pub data: Vec<ResponseSummaryFinancialStatusProjects>,
    pub total_records: usize,
    pub total_contract_cost: i64,
    pub total_paid: i64,
    pub total_balance: i64,
}

#[derive(Serialize, Default, TS, Deserialize, FromQueryResult, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryFinancialStatusReports {
    pub total_projects: i64,
    pub total_appropriation: i64,
    pub total_contract_cost: i64,
    pub total_paid: i64,
    pub total_balance: i64,
    pub total_savings: i64,
}

#[derive(Serialize, Default, TS, Deserialize, Clone)]
#[ts(export, export_to = "../../../client/src/types/Reports.ts")]
pub struct ResponseSummaryFinancialStatusReportsOverview {
    pub data_implemented: ResponseSummaryFinancialStatusReports,
    pub data_unimplemented: ResponseSummaryFinancialStatusReports,
    pub data_suspended: ResponseSummaryFinancialStatusReports,
    pub data_summary: ResponseSummaryFinancialStatusReports
}