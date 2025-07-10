use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{settings::settings_sector, user};
use sea_orm::{ActiveValue::Set, prelude::*};
use services::{db_connection::DB, redis::Redis, request::settings::{RequestAddSector, RequestDeleteSector, RequestUpdateSector}};

pub async fn get_all_sector(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("settings_get_all_sector".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = settings_sector::Entity::find().all(&db.db_connection).await.unwrap();

    redis.stored_value(&serde_json::to_string(&response).unwrap()).await.unwrap();

    (StatusCode::OK, Json(response))

}

pub async fn get_sector_by_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {

    let mut redis = Redis::new(format!("get_sector_by_id_{}", id), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = settings_sector::Entity::find_by_id(id).one(&db.db_connection).await.unwrap();

    if response.is_none() {
        return (StatusCode::OK, Json::default());
    }

    redis.stored_value(&serde_json::to_string(&response.as_ref().unwrap()).unwrap()).await.unwrap();

    (StatusCode::OK, Json(response.unwrap()))

}

pub async fn add_sector(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddSector>
) -> impl IntoResponse {

    let make_add = settings_sector::ActiveModel {
        name: Set(request.name),
        ..Default::default()
    };

    let response = settings_sector::Entity::insert(make_add).exec(&db.db_connection).await;

    Redis::new("settings_get_all_sector".to_string(), db.redis_connection.clone())
        .remove_value_if_exists().await.unwrap();

    match response {
        Ok(response) => (StatusCode::OK, format!("{}", response.last_insert_id)),
        Err(_) => (StatusCode::OK, format!(""))
    }
}

pub async fn update_sector(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateSector>
) -> impl IntoResponse {

    let get_sector = settings_sector::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_sector.is_none() {
        return (StatusCode::OK, Json::default());
    }

    let mut sector: settings_sector::ActiveModel = get_sector.unwrap().into();

    sector.name = Set(request.name);

    let response = sector.update(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_sector".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![format!("get_sector_by_id_{}", request.id).as_str()]).await.unwrap();

    (StatusCode::OK, Json(response))
}

pub async fn delete_sector_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteSector>
) -> impl IntoResponse {

    let get_account_by_hashed_password = user::Entity::find()
        .filter(user::Column::Password.eq(request.hash_request_password()))
        .one(&db.db_connection).await.unwrap();

    if get_account_by_hashed_password.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let response = settings_sector::Entity::delete_by_id(request.id).exec(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_sector".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![format!("get_sector_by_id_{}", request.id).as_str()]).await.unwrap();

    (StatusCode::OK, format!("{}", response.rows_affected))

}