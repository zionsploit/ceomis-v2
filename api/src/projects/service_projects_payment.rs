use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{projects_payment, user};
use sea_orm::{ActiveValue::Set, EntityTrait, prelude::*};
use services::{db_connection::DB, redis::Redis, request::projects::{RequestAddProjectPayment, RequestDeleteProjectsPayment}};

pub async fn add_projects_payment(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddProjectPayment>
) -> impl IntoResponse {
    
    let make_add = projects_payment::ActiveModel {
        project_id: Set(request.project_id),
        billing_date: Set(request.billing_date),
        processed_by: Set(request.processed_by),
        amount_due: Set(request.amount_due),
        amount_paid: Set(request.amount_paid),
        reference_no: Set(request.reference_no),
        payment_date: Set(request.payment_date),
        ..Default::default()
    };

    let response = projects_payment::Entity::insert(make_add).exec(&db.db_connection).await.unwrap();

    Redis::new(format!("get_projects_by_id_{}", request.project_id), db.redis_connection.clone())
        .remove_value_if_exists().await.unwrap();

    (StatusCode::CREATED, format!("{}", response.last_insert_id))
}

pub async fn remove_projects_payment_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteProjectsPayment>
) -> impl IntoResponse {

    if user::Entity::find().filter(user::Column::Password.eq(request.hash_password())).one(&db.db_connection).await.unwrap().is_none() {
        return (StatusCode::NOT_FOUND, format!(""));
    }

    let get_payment = projects_payment::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_payment.is_none() {
        return (StatusCode::NOT_FOUND, format!(""));
    }

     Redis::new(format!("get_projects_by_id_{}", get_payment.as_ref().unwrap().project_id), db.redis_connection.clone())
        .remove_value_if_exists().await.unwrap();

    let response = get_payment.unwrap().delete(&db.db_connection).await.unwrap();

    (StatusCode::CREATED, format!("{}", response.rows_affected))

}