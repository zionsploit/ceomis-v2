use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use entity::user_roles;
use sea_orm::EntityTrait;
use services::{db_connection::DB, redis::Redis};

pub async fn get_all_roles(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_all_roles".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = user_roles::Entity::find().all(&db.db_connection).await.unwrap();

    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(response))
}