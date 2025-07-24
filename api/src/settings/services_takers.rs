use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{settings::settings_takers, user};
use sea_orm::{prelude::*, ActiveValue::Set, QueryOrder};
use services::{db_connection::DB, redis::Redis, request::settings::{RequestAddTakers, RequestDeleteTakers, RequestUpdateTakers}};

pub async fn get_all_takers(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("settings_get_all_takers".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }
    
    let response = settings_takers::Entity::find().order_by_asc(settings_takers::Column::Name).all(&db.db_connection).await.unwrap();

    if response.len().gt(&0) {
        redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();
    }

    (StatusCode::OK, Json(response))
}

pub async fn get_takers_by_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {

    let mut redis = Redis::new(format!("get_takers_by_id_{}", id), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = settings_takers::Entity::find_by_id(id).one(&db.db_connection).await.unwrap();

    if response.is_none() {
        return (StatusCode::OK, Json::default());
    }

    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(response.unwrap()))
    
}

pub async fn add_takers(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddTakers>
) -> impl IntoResponse {

    let make_add = settings_takers::ActiveModel {
        name: Set(request.name),
        contact_number: Set(request.contact_number),
        ..Default::default()
    };

    let response = settings_takers::Entity::insert(make_add).exec(&db.db_connection).await;

    if response.is_err() {
        return (StatusCode::OK, format!(""));
    }

    Redis::new("settings_get_all_takers".to_string(), db.redis_connection.clone())
        .remove_value().await.unwrap();

    (StatusCode::OK, format!("{}", response.unwrap().last_insert_id))
}

pub async fn update_takers(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateTakers>
) -> impl IntoResponse {

    let get_takers_data = settings_takers::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();


    let mut takers_data: settings_takers::ActiveModel = get_takers_data.unwrap().into();

    takers_data.name = Set(request.name);
    takers_data.contact_number = Set(request.contact_number);

    let response = takers_data.update(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_takers".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![
            format!("get_takers_by_id_{}", request.id).as_str()
        ]).await.unwrap();

    (StatusCode::OK, Json(response))
}

pub async fn delete_takers_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteTakers>
) -> impl IntoResponse {

    let get_account_by_hashed = user::Entity::find()
        .filter(user::Column::Password.eq(request.hash_request_password())).one(&db.db_connection).await.unwrap();

    if get_account_by_hashed.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let response = settings_takers::Entity::delete_by_id(request.id).exec(&db.db_connection).await.unwrap();
    
    Redis::new("settings_get_all_takers".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![
            format!("get_takers_by_id_{}", request.id).as_str()
        ]).await.unwrap();

    (StatusCode::OK, format!("{}", response.rows_affected))

}