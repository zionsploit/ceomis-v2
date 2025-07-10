use std::{collections::HashMap, sync::Arc};
use sea_orm::{prelude::*, ActiveValue::Set, DeleteResult, QueryOrder};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{settings::settings_sdg::{self}, user};
use services::{db_connection::DB, redis::Redis, request::settings::{RequestAddSustainableDevelopmentGoals, RequestDeleteSustainableDevelopmentGoals, RequestUpdateSustainableDevelopmentGoals}};


pub async fn get_all(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {
    let mut redis = Redis::new("settings_get_all_sdg".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let response: Vec<settings_sdg::Model> = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json::from(response));
    }

    let response: Vec<settings_sdg::Model> = settings_sdg::Entity::find().order_by_asc(settings_sdg::Column::Id).all(&db.db_connection).await.unwrap();

    if response.len().gt(&0) {
        let json_data = serde_json::to_string(&response).unwrap();
        redis.stored_value(&json_data).await.unwrap();
    }

    (StatusCode::OK, Json::from(response))
}

pub async fn get_sdg_by_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let mut redis = Redis::new(format!("get_sdg_by_id_{}", id).to_string(), db.redis_connection.clone());

    if let Some(redis_value) = redis.get_value().await {
        let json_data: settings_sdg::Model = serde_json::from_str(&redis_value).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let get_sdg_by_id = settings_sdg::Entity::find_by_id( id).one(&db.db_connection).await.unwrap();

    if get_sdg_by_id.is_none() {
        return (StatusCode::OK, Json::default());
    }

    redis.stored_value(&serde_json::to_string(&get_sdg_by_id.clone().unwrap()).unwrap()).await.unwrap();

    (StatusCode::OK, Json(get_sdg_by_id.unwrap()))

}

pub async fn add_sdg(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddSustainableDevelopmentGoals>
) -> impl IntoResponse {

    let make_add = settings_sdg::ActiveModel {
        name: Set(request.name),
        ..Default::default()
    };

    let response = settings_sdg::Entity::insert(make_add).exec(&db.db_connection).await;

    Redis::new("settings_get_all_sdg".to_string(), db.redis_connection.clone())
        .remove_value_if_exists().await.unwrap();

    match response {
        Ok(res) => (StatusCode::OK, format!("{:?}", res.last_insert_id)),
        Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, format!(""))
    }
}

pub async fn update_sdg(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateSustainableDevelopmentGoals>
) -> impl IntoResponse {

    let get_sdg: Option<settings_sdg::Model> = settings_sdg::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if let None = get_sdg {
        return (StatusCode::OK, Json::default())
    }

    let mut sdg: settings_sdg::ActiveModel = get_sdg.unwrap().into();

    sdg.name = Set(request.name);

    let update_sdg: settings_sdg::Model = sdg.update(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_sdg".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![format!("get_sdg_by_id_{}", request.id).as_str()]).await.unwrap();

    (StatusCode::OK, Json::from(HashMap::from([("id", update_sdg.id.to_string()), ("name", update_sdg.name)])))
}

pub async fn delete_sdg_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteSustainableDevelopmentGoals>
) -> impl IntoResponse {

    let get_account_by_hashed_password: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Password.eq(request.hash_request_password()))
        .one(&db.db_connection).await.unwrap();

    if get_account_by_hashed_password.is_none() {
        return (StatusCode::OK, format!(""));
    }

    let get_sdg: Option<settings_sdg::Model> = settings_sdg::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if let None = get_sdg {
        return (StatusCode::OK, format!(""));
    }

    let sdg: settings_sdg::Model = get_sdg.unwrap().into();

    let delete_result: DeleteResult = sdg.delete(&db.db_connection).await.unwrap();

    Redis::new("settings_get_all_sdg".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![format!("get_sdg_by_id_{}", request.id).as_str()]).await.unwrap();

    (StatusCode::OK, format!("{:?}", delete_result.rows_affected))
}