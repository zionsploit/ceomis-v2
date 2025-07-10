use axum::{routing::{get, post}, Router};

use crate::settings::{services_barangay::{add_barangay, delete_barangay_by_id, get_all_barangays, get_barangay_by_id, update_barangay}, services_categories::{add_categories, delete_categories_by_id, get_all_categories, get_categories_by_id, update_categories}, services_incharge::{add_incharge, delete_inchage_by_id, get_all_incharge, get_incharge_by_id, update_incharge}, services_sdg::{add_sdg, delete_sdg_by_id, get_all, get_sdg_by_id, update_sdg}, services_sector::{add_sector, delete_sector_by_id, get_all_sector, get_sector_by_id, update_sector}, services_sof::{add_sof, delete_sof_by_id, get_all_sof, get_sof_by_id, update_sof}, services_takers::{add_takers, delete_takers_by_id, get_all_takers, get_takers_by_id, update_takers}, services_type::{add_type, delete_types_by_id, get_all_type, get_type_by_id, update_type}};

pub mod services_sdg;
pub mod services_sof;
pub mod services_type;
pub mod services_incharge;
pub mod services_categories;
pub mod services_sector;
pub mod services_barangay;
pub mod services_takers;

pub fn api_settings() -> Router {
    Router::new()
        .nest("/sdg", 
            Router::new()
                .route("/", get(get_all))
                .route("/{id}", get(get_sdg_by_id))
                .route("/add", post(add_sdg))
                .route("/update", post(update_sdg))
                .route("/delete", post(delete_sdg_by_id))
        )
        .nest("/sof",
            Router::new()
                .route("/", get(get_all_sof))
                .route("/{id}", get(get_sof_by_id))
                .route("/add", post(add_sof))
                .route("/update", post(update_sof))
                .route("/delete", post(delete_sof_by_id))
        )
        .nest("/type", 
            Router::new()
                .route("/", get(get_all_type))
                .route("/{id}", get(get_type_by_id))
                .route("/add", post(add_type))
                .route("/update", post(update_type))
                .route("/delete", post(delete_types_by_id))
        )
        .nest("/incharge", 
            Router::new()
                .route("/", get(get_all_incharge))
                .route("/{id}", get(get_incharge_by_id))
                .route("/add", post(add_incharge))
                .route("/update", post(update_incharge))
                .route("/delete", post(delete_inchage_by_id))
        )
        .nest("/categories",
            Router::new()
                .route("/", get(get_all_categories))
                .route("/{id}", get(get_categories_by_id))
                .route("/add", post(add_categories))
                .route("/update", post(update_categories))
                .route("/delete", post(delete_categories_by_id))
        )
        .nest("/sector", 
            Router::new()
                .route("/", get(get_all_sector))
                .route("/{id}", get(get_sector_by_id))
                .route("/add", post(add_sector))
                .route("/update", post(update_sector))
                .route("/delete", post(delete_sector_by_id))
        )
        .nest("/barangay", 
            Router::new()
                .route("/", get(get_all_barangays))
                .route("/{id}", get(get_barangay_by_id))
                .route("/add", post(add_barangay))
                .route("/update", post(update_barangay))
                .route("/delete", post(delete_barangay_by_id))
        )
        .nest("/takers", 
            Router::new()
                .route("/", get(get_all_takers))
                .route("/{id}", get(get_takers_by_id))
                .route("/add", post(add_takers))
                .route("/update", post(update_takers))
                .route("/delete", post(delete_takers_by_id))
        )
}