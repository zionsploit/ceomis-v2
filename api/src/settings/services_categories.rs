use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{settings::settings_categories, user};
use sea_orm::{ActiveValue::Set, prelude::*};
use services::{db_connection::DB, redis::Redis, request::settings::{RequestAddCategories, RequestDeleteCategories, RequestUpdateCategories}};

pub async fn get_all_categories(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("settings_get_all_categories".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data))
    }

    let get_all_categories = settings_categories::Entity::find().all(&db.db_connection).await.unwrap();

    redis.stored_value(&serde_json::to_string(&get_all_categories).unwrap()).await.unwrap();

    (StatusCode::OK, Json(get_all_categories))

}

pub async fn get_categories_by_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {

    let mut redis = Redis::new(format!("get_categories_by_id_{}", id), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let get_categories_by_id = settings_categories::Entity::find_by_id(id).one(&db.db_connection).await.unwrap();

    if get_categories_by_id.is_none() {
        return (StatusCode::OK, Json::default());
    }

    redis.stored_value(&serde_json::to_string(&get_categories_by_id).unwrap()).await.unwrap();

    (StatusCode::OK, Json(get_categories_by_id.unwrap()))

}

pub async fn add_categories(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddCategories>
) -> impl IntoResponse {

    let make_add = settings_categories::ActiveModel {
        name: Set(request.name),
        ..Default::default()
    };

    let response = settings_categories::Entity::insert(make_add).exec(&db.db_connection).await;

    Redis::new("settings_get_all_categories".to_string(), db.redis_connection.clone())
        .remove_value().await.unwrap();

    match response {
        Ok(response) => (StatusCode::OK, format!("{}", response.last_insert_id)),
        Err(_) => (StatusCode::OK, format!(""))
    }
}

pub async fn update_categories(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateCategories>
) -> impl IntoResponse {

    let get_categories = settings_categories::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_categories.is_none() {
        return (StatusCode::OK, Json::default());
    }

    let mut categoris: settings_categories::ActiveModel = get_categories.unwrap().into();

    categoris.name = Set(request.name);

    let response = categoris.update(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_categories".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![format!("get_categories_by_id_{}", request.id).as_str()]).await.unwrap();

    (StatusCode::OK, Json(response))
    
}

pub async fn delete_categories_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteCategories>
) -> impl IntoResponse {

    let get_account_by_hashed_password = user::Entity::find()
        .filter(user::Column::Password.eq(request.hash_request_password())).one(&db.db_connection).await.unwrap();

    if get_account_by_hashed_password.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let get_categories = settings_categories::Entity::delete_by_id(request.id).exec(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_categories".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![format!("get_categories_by_id_{}", request.id).as_str()]).await.unwrap();
    
    (StatusCode::OK, format!("{}", get_categories.rows_affected))

}