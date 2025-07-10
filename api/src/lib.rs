mod user;
mod settings;
mod contractors;
mod projects;
mod reports;
use axum::{routing::get, Router};
use files_api::projects_files::project_files_api;
use ::reports::api_generate_reports;

use crate::{contractors::api_contractors, projects::api_projects, reports::api_reports, settings::api_settings, user::api_user};

pub fn api_route() -> Router {
    Router::new()
        .route("/", get("HELLO API ROUTES"))
        .nest("/users", api_user())
        .nest("/settings", api_settings())
        .nest("/contractors", api_contractors())
        .nest("/projects", api_projects())
        .nest("/projects-files", project_files_api())
        .nest("/reports", api_reports())
        .nest("/generate-reports", api_generate_reports())
}