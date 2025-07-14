use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{settings::settings_type, user};
use sea_orm::{ActiveValue::Set, prelude::*};
use services::{db_connection::DB, redis::Redis, request::settings::{RequestAddTypes, RequestDeleteTypes, RequestUpdateTypes}};

pub async fn get_all_type(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {
    let mut redis = Redis::new("settings_get_all_type".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let response: Vec<settings_type::Model> = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json::from(response));
    }

    let response: Vec<settings_type::Model> = settings_type::Entity::find().all(&db.db_connection).await.unwrap();

    if response.len().gt(&0) {

        let x = &response;

        let json_data = serde_json::to_string(&x).unwrap();

        redis.stored_value(&json_data, None).await.unwrap();
    }

    (StatusCode::OK, Json::from(response))

}
    
pub async fn get_type_by_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let mut redis = Redis::new(format!("get_type_by_id_{}", id).to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data: settings_type::Model = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let get_sof_by_id = settings_type::Entity::find_by_id(id).one(&db.db_connection).await.unwrap();

    if get_sof_by_id.is_none() {
        return (StatusCode::OK, Json::default());
    }

    redis.stored_value(&serde_json::to_string(&get_sof_by_id.clone().unwrap()).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(get_sof_by_id.unwrap()))
}

pub async fn add_type(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddTypes>
) -> impl IntoResponse {

    let make_add = settings_type::ActiveModel {
        name: Set(request.name),
        ..Default::default()
    };


    let response = settings_type::Entity::insert(make_add).exec(&db.db_connection).await;
    
    Redis::new("settings_get_all_type".to_string(), db.redis_connection.clone())
        .remove_value_if_exists().await.unwrap();

    match response {
        Ok(res) => (StatusCode::OK, format!("{}", res.last_insert_id)),
        Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, format!(""))
    }
}

pub async fn update_type(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateTypes>
) -> impl IntoResponse {
    
    let get_type: Option<settings_type::Model> = settings_type::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_type.is_none() {
        return (StatusCode::OK, Json::default());
    }

    let mut stype: settings_type::ActiveModel = get_type.unwrap().into();

    stype.name = Set(request.name);

    let update_type: settings_type::Model = stype.update(&db.db_connection).await.unwrap();
    
    Redis::new("settings_get_all_type".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![&format!("get_type_by_id_{}", request.id.to_string())]).await.unwrap();

    (StatusCode::OK, Json::from(update_type))

}

pub async fn delete_types_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteTypes>
) -> impl IntoResponse {

    let get_account_by_hashed_password = user::Entity::find()
        .filter(user::Column::Password.eq(request.hash_request_password()))
        .one(&db.db_connection).await.unwrap();

    if get_account_by_hashed_password.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let get_type = settings_type::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_type.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let stype: settings_type::ActiveModel = get_type.unwrap().into();

    let delete_type = stype.delete(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_type".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![&format!("get_type_by_id_{}", request.id.to_string())]).await.unwrap();

    (StatusCode::OK, format!("{}", delete_type.rows_affected))

}