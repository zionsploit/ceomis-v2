use axum::{routing::{get, post}, Router};

use crate::projects::{service_projects::{add_projects, delete_projects_remarks, dispose_projects_by_id, get_prepare_add_projects, get_projects_appropriation_by_sector, get_projects_by_fund, get_projects_by_id, get_projects_infra_code_by_projects_id, get_projects_stats_by_category, get_projects_stats_by_types, get_projects_stats_overview, get_projects_top_10_by_appropriation, get_projects_top_10_by_awarded_contractors, update_projects, upsert_projects_infra_code}, service_projects_payment::{add_projects_payment, remove_projects_payment_by_id}};

mod service_projects;
mod service_projects_payment;

pub fn api_projects () -> Router {
    Router::new()
        // GET
        .route("/{id}", get(get_projects_by_id))
        .route("/stats-overview", get(get_projects_stats_overview))
        .route("/stats-types", get(get_projects_stats_by_types))
        .route("/stats-category", get(get_projects_stats_by_category))
        .route("/appropriation-by-sector", get(get_projects_appropriation_by_sector))
        .route("/top-10-by-appropriation", get(get_projects_top_10_by_appropriation))
        .route("/top-1o-by-awarded-contractors", get(get_projects_top_10_by_awarded_contractors))
        .route("/get-by-fund/{id}", get(get_projects_by_fund))
        .route("/prepare-add-projects", get(get_prepare_add_projects))
        .route("/get-infra-code-by-project-id/{id}", get(get_projects_infra_code_by_projects_id))
        // POST 
        .route("/add", post(add_projects))
        .route("/update", post(update_projects))
        .route("/disposed", post(dispose_projects_by_id))
        .route("/upsert-projects-info-code", post(upsert_projects_infra_code))
        .route("/delete-project-remarks", post(delete_projects_remarks))
        // POST PROJECT PAYMENT
        .route("/add-payment", post(add_projects_payment))
        .route("/remove-payment", post(remove_projects_payment_by_id))
}