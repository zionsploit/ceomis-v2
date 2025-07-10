pub mod projects;
use axum::{routing::{get, post}, Router};

use crate::projects_files::projects::add_projects_remarks;

pub fn project_files_api() -> Router {
    Router::new()
        .route("/", get(|| async { "HELLO WORLD" }))
        .route("/remarks", post(add_projects_remarks)) // max at 50MB
}