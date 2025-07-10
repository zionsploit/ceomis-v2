use axum::{routing::get, Router};

use crate::reports::service_reports::{summary_financial_status_project, summary_financial_status_report, summary_implementation_by_year, summary_list_of_projects, summary_project_savings_report, summary_projects_per_type, summary_slippage_report};

pub mod service_reports;

pub fn api_reports() -> Router {
    Router::new()
        .route("/summary_list_of_projects", get(summary_list_of_projects))
        .route("/summary_implementation_by_year", get(summary_implementation_by_year))
        .route("/summary_projects_per_type", get(summary_projects_per_type))
        .route("/summary_project_savings_report", get(summary_project_savings_report))
        .route("/summary_slippage_report", get(summary_slippage_report))
        .route("/summary_financial_status_project", get(summary_financial_status_project))
        .route("/summary_financial_status_report", get(summary_financial_status_report))
}