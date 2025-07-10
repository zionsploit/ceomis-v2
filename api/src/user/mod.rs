mod services_user;
mod services_roles;

use axum::{routing::{get, post}, Router};
use services_user::{create_user_account, create_user_info_account, post_login};

use crate::user::{services_roles::get_all_roles, services_user::{create_update_user_info_by_id, delete_user, get_all_account_info, get_all_users, get_all_users_with_roles, get_users_all_info_by_id, update_user_by_id}};


pub fn api_user() -> Router {
    Router::new()
        .route("/get-all-roles", get(get_all_roles))
        .route("/get-all-user", get(get_all_users))
        .route("/get-full-users/{id}", get(get_users_all_info_by_id))
        .route("/get-all-users-with-roles", get(get_all_users_with_roles))
        .route("/get-all-account-info", get(get_all_account_info))
        .route("/create", post(create_user_account))
        .route("/create-account-info", post(create_user_info_account))
        .route("/login", post(post_login))
        .route("/update-user-info-by-id", post(create_update_user_info_by_id))
        .route("/update-user-by-id", post(update_user_by_id))
        .route("/delete-user-by-id", post(delete_user))
}