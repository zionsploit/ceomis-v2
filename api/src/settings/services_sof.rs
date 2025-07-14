use std::{collections::HashMap, sync::Arc};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{settings::settings_sof, user};
use sea_orm::{prelude::*, ActiveValue::Set, DeleteResult, QueryOrder};
use services::{db_connection::DB, redis::Redis, request::settings::{RequestAddSourceOfFunds, RequestDeleteSourceOfFunds, RequestUpdateSourceOfFunds}};

pub async fn get_all_sof(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {
    let mut redis = Redis::new("settings_get_all_sof".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let response: Vec<settings_sof::Model> = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json::from(response));
    }

    let response: Vec<settings_sof::Model> = settings_sof::Entity::find().order_by_asc(settings_sof::Column::Id).all(&db.db_connection).await.unwrap();

    if response.len().gt(&0) {
        let json_data = serde_json::to_string(&response).unwrap();
        redis.stored_value(&json_data, None).await.unwrap();
    }

    (StatusCode::OK, Json::from(response))
}

pub async fn get_sof_by_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let mut redis = Redis::new(format!("get_sof_by_id_{}", id).to_string(), db.redis_connection.clone());

    if let Some(redis_value) = redis.get_value().await {
        let json_data: settings_sof::Model = serde_json::from_str(&redis_value).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let get_sof_by_id: Option<settings_sof::Model> = settings_sof::Entity::find_by_id(id).one(&db.db_connection).await.unwrap();

    if get_sof_by_id.is_none() {
        return (StatusCode::OK, Json::default());
    }

    redis.stored_value(&serde_json::to_string(&get_sof_by_id.clone().unwrap()).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(get_sof_by_id.unwrap()))
}

pub async fn add_sof(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddSourceOfFunds>
) -> impl IntoResponse {
    
    let make_add = settings_sof::ActiveModel {
        name: Set(request.name),
        ..Default::default()
    };

    let response = settings_sof::Entity::insert(make_add).exec(&db.db_connection).await;

    Redis::new("settings_get_all_sof".to_string(), db.redis_connection.clone())
        .remove_value_if_exists().await.unwrap();

    match response {
        Ok(res) => (StatusCode::OK, format!("{}", res.last_insert_id)),
        Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, format!(""))
    }
}

pub async fn update_sof(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateSourceOfFunds>
) -> impl IntoResponse {

    let get_sof: Option<settings_sof::Model> = settings_sof::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_sof.is_none() {
        return (StatusCode::OK, Json::default());
    }

    let mut sof: settings_sof::ActiveModel = get_sof.unwrap().into();

    sof.name = Set(request.name);

    let update_sof = sof.update(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_sof".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![&format!("get_sof_by_id_{}", request.id.to_string())]).await.unwrap();

    (StatusCode::OK, Json::from(HashMap::from([("id", update_sof.id.to_string()), ("name", update_sof.name)])))
}

pub async fn delete_sof_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteSourceOfFunds>
) -> impl IntoResponse {

    let get_account_by_hashed_password: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Password.eq(request.hash_request_password()))
        .one(&db.db_connection).await.unwrap();

    if get_account_by_hashed_password.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let get_sof: Option<settings_sof::Model> = settings_sof::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_sof.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let sof: settings_sof::Model = get_sof.unwrap().into();

    let delete_result: DeleteResult = sof.delete(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_sof".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![&format!("get_sof_by_id_{}", request.id)]).await.unwrap();

    (StatusCode::OK, format!("{:?}", delete_result.rows_affected))


}