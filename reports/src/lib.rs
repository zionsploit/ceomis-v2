use axum::{routing::post, Router};

use crate::{contractors::reports_contractors_projects::reports_contractors_projects_by_id, projects::{reports_financial_per_projects::reports_financial_per_projects, reports_financial_status::reports_financial_status, reports_projects::reports_projects_by_id, reports_projects_remarks::reports_projects_remarks_by_id, reports_slippage_projects::reports_slippage_projects, summary_implementation_by_year::reports_summary_implementation_by_year, summary_list_of_projects::reports_summary_list_of_projects, summary_projects_per_type::reports_summary_projects_per_type, summary_projects_savings::reports_summary_projects_savings}};

pub mod projects;
pub mod helpers;
pub mod contractors;

pub fn api_generate_reports() -> Router {
    Router::new()
        .route("/projects-by-id", post(reports_projects_by_id))
        .route("/projects-remarks-by-id", post(reports_projects_remarks_by_id))
        // CONTRACTORS
        .route("/contractor-projects-by-id", post(reports_contractors_projects_by_id))
        // Actual Reports
        .route("/reports_summary_list_of_projects", post(reports_summary_list_of_projects))
        .route("/reports_summary_implementation_by_year", post(reports_summary_implementation_by_year))
        .route("/reports_summary_projects_per_type", post(reports_summary_projects_per_type))
        .route("/reports_summary_projects_savings", post(reports_summary_projects_savings))
        .route("/reports_slippage_projects", post(reports_slippage_projects))
        .route("/reports_financial_per_projects", post(reports_financial_per_projects))
        .route("/reports_financial_status", post(reports_financial_status))
}