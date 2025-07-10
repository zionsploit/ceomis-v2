use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{settings::settings_incharge::{self}, user};
use sea_orm::{prelude::*, ActiveValue::Set};
use services::{db_connection::DB, redis::Redis, request::settings::{RequestAddIncharge, RequestDeleteIncharge, RequestUpdateIncharge}};

pub async fn get_all_incharge(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("settings_get_all_incharge".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let get_all_incharge = settings_incharge::Entity::find().all(&db.db_connection).await.unwrap();

    redis.stored_value(&serde_json::to_string(&get_all_incharge).unwrap()).await.unwrap();

    (StatusCode::OK, Json(get_all_incharge))

}

pub async fn get_incharge_by_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {

    let mut redis = Redis::new(format!("get_incharge_by_id_{}", id).to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let get_incharge_by_id = settings_incharge::Entity::find_by_id(id).one(&db.db_connection).await.unwrap();

    if get_incharge_by_id.is_none() {
        return (StatusCode::OK, Json::default());
    }

    redis.stored_value(&serde_json::to_string(&get_incharge_by_id).unwrap()).await.unwrap();

    (StatusCode::OK, Json(get_incharge_by_id.unwrap()))

}

pub async fn add_incharge(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddIncharge>,
) -> impl IntoResponse {

    let make_add = settings_incharge::ActiveModel {
        name: Set(request.name),
        ..Default::default()
    };

    let response = settings_incharge::Entity::insert(make_add).exec(&db.db_connection).await;

    Redis::new("settings_get_all_incharge".to_string(), db.redis_connection.clone())
        .remove_value_if_exists().await.unwrap();

    match response {
        Ok(response) => (StatusCode::OK, format!("{}", response.last_insert_id)),
        Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, format!(""))
    }
}

pub async fn update_incharge(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateIncharge>
) -> impl IntoResponse {

    let get_incharge = settings_incharge::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_incharge.is_none() {
        return (StatusCode::OK, Json::default());
    }

    let mut incharge: settings_incharge::ActiveModel = get_incharge.unwrap().into();

    incharge.name = Set(request.name);

    let response = incharge.update(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_incharge".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![format!("get_incharge_by_id_{}", request.id).as_str()]).await.unwrap();

    (StatusCode::OK, Json(response))
}

pub async fn delete_inchage_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteIncharge>
) -> impl IntoResponse {

    let get_account_by_hashed_password = user::Entity::find()
        .filter(user::Column::Password.eq(request.hash_request_password()))
        .one(&db.db_connection).await.unwrap();

    if get_account_by_hashed_password.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let get_incharge = settings_incharge::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_incharge.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let delete_incharge = get_incharge.unwrap().delete(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_incharge".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![format!("get_incharge_by_id_{}", request.id).as_str()]).await.unwrap();

    (StatusCode::OK, format!("{}", delete_incharge.rows_affected))

}