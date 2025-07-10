use axum::{routing::{get, post}, Router};

use crate::contractors::service_contractors::{add_contractors, delete_contractors_by_id, get_all_contractors, get_contractors_by_id, update_contractors};

mod service_contractors;

pub fn api_contractors() -> Router {
    Router::new()
        .route("/", get(get_all_contractors))
        .route("/{id}", get(get_contractors_by_id))
        .route("/add", post(add_contractors))
        .route("/update", post(update_contractors))
        .route("/delete", post(delete_contractors_by_id))
}