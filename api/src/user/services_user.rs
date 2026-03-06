use std::sync::Arc;

use axum::{extract::Path, http::{header, StatusCode}, response::{IntoResponse, Response}, Extension, Json};
use entity::{user::{self, ILoginUsersFromQuery}, user_info, user_roles};
use sea_orm::{prelude::*, ActiveValue::{NotSet, Set}, InsertResult, IntoActiveModel, QueryOrder, QuerySelect};
use services::{db_connection::DB, redis::Redis, request::user::{RequestAddUpdateUserInfoById, RequestAddUser, RequestDeleteUserById, RequestUpdateUser, RequestUserInfo, RequestUserLogin}, response::user::{ResponseAccountInfo, ResponseLogin, ResponseUsers, ResponseUsersWithFullInfo, ResponseUsersWithRoles}};


// GET REQUEST

pub async fn get_users_all_info_by_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {

    let mut redis = Redis::new(format!("get_users_all_info_by_id_{}", id), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }   

    let response = user::Entity::find_by_id(id)
        .left_join(user_roles::Entity)
        .left_join(user_info::Entity)
        .select_only()
        .column_as(user::Column::Id, "user_id")
        .column_as(user::Column::Email, "user_email")
        .column_as(user_roles::Column::Id, "role_id")
        .column_as(user_roles::Column::Name, "role_name")
        .column_as(user_info::Column::Id, "user_info_id")
        .column_as(user_info::Column::FirstName, "user_info_first_name")
        .column_as(user_info::Column::MiddleName, "user_info_middle_name")
        .column_as(user_info::Column::LastName, "user_info_last_name")
        .filter(user::Column::IsDelete.eq(false))
        .into_model::<ResponseUsersWithFullInfo>()
        .one(&db.db_connection).await.unwrap();

    if response.is_none() {
        return (StatusCode::OK, Json::default());
    }

    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(response))
}

pub async fn get_all_users_with_roles(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_all_users_with_roles".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data))
    }

    let response = user::Entity::find()
        .select_only()
        .column_as(user::Column::Id, "user_id")
        .column(user::Column::Email)
        .column(user::Column::UserRolesId)
        .column(user_roles::Column::Id)
        .column(user_roles::Column::Name)
        .left_join(user_roles::Entity)
        .order_by_asc(user::Column::Id)
        .filter(user::Column::IsDelete.eq(false))
        .into_model::<ResponseUsersWithRoles>()
        .all(&db.db_connection)
        .await.unwrap();


    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(response))

}

pub async fn get_all_users(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_all_users".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response: Vec<ResponseUsers> = user::Entity::find()
        .select_only()
        .columns([
            user::Column::Id,
            user::Column::Email
        ])
        .filter(user::Column::IsDelete.eq(false)).into_model::<ResponseUsers>().all(&db.db_connection).await.unwrap();

    
    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(response))

}

pub async fn get_all_account_info(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_all_account_info".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = user_info::Entity::find()
        .select_only()
        .columns([
            user_info::Column::Id,
            user_info::Column::FirstName,
            user_info::Column::MiddleName,
            user_info::Column::LastName,
        ]).into_model::<ResponseAccountInfo>().all(&db.db_connection).await.unwrap();

    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(response))

}

// POST REQUEST
pub async fn update_user_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateUser>
) -> impl IntoResponse {

    let make_find_users = user::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if make_find_users.is_none() {
        return (StatusCode::NOT_FOUND, format!(""));
    }

    let mut users: user::ActiveModel = make_find_users.unwrap().into();

    users.email = Set(request.email.to_owned());
    users.password = match request.password {
            Some(_) => Set(request.clone().hash_password()),
            None => NotSet
        };
    users.user_roles_id = Set(request.user_role);

    let response = users.update(&db.db_connection).await;

    match response {
        Ok(_) => {
            Redis::new("get_all_users".to_string(), db.redis_connection.clone())
            .remove_multi_value(vec![
                "get_all_users_with_roles", 
                "get_all_account_info", 
                format!("get_users_all_info_by_id_{}", request.id).as_str()
            ]).await.unwrap();

            return (StatusCode::CREATED, format!("OK"));
        },
        Err(_) => (StatusCode::NOT_FOUND, format!(""))
    }
}

pub async fn create_update_user_info_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddUpdateUserInfoById>
) -> impl IntoResponse {

    let make_model = user_info::ActiveModel {
        id: match request.id {
            Some(id) => Set(id),
            None => NotSet
        },
        first_name: Set(request.first_name),
        middle_name: Set(request.middle_name),
        last_name: Set(request.last_name),
        user_id: Set(request.user_id)
    };

    let response = make_model.save(&db.db_connection).await;

    if response.is_ok() {

        let response_user_id = &response.unwrap().user_id.into_value().unwrap();

        Redis::new("get_all_users".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![
            "get_all_users_with_roles", 
            "get_all_account_info", 
            format!("get_users_all_info_by_id_{}", response_user_id).as_str()
        ]).await.unwrap();

        return (StatusCode::CREATED, format!("OK"));
    } else {
        return (StatusCode::CREATED, format!(""));
    }

}

pub async fn create_user_account(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddUser>
) -> impl IntoResponse {

    let make_user = user::ActiveModel {
        email: Set(request.email.clone()),
        password: Set(request.hash_password()),
        user_roles_id: Set(request.user_role),
        ..Default::default()
    };

    let res: InsertResult<user::ActiveModel> = user::Entity::insert(make_user).exec(&db.db_connection).await.unwrap();
    
    Redis::new("get_all_users".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![
            "get_all_users_with_roles", "get_all_account_info"
        ]).await.unwrap();

    (StatusCode::CREATED, format!("{:?}", res))
}

pub async fn create_user_info_account (
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUserInfo>
) -> impl IntoResponse {

    let make_user_info = user_info::ActiveModel {
        first_name: Set(request.first_name.clone()),
        middle_name: Set(request.middle_name.clone()),
        last_name: Set(request.last_name.clone()),
        user_id: Set(request.user_id),
        ..Default::default()
    };

    let res = user_info::Entity::insert(make_user_info).exec(&db.db_connection).await.unwrap();

    Redis::new("get_all_users".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![
            "get_all_users_with_roles", "get_all_account_info"
        ]).await.unwrap();

    (StatusCode::OK, format!("{:?}", res))
}

pub async fn post_login(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUserLogin>
) -> impl IntoResponse {

    let find_users = user::Entity::find()
        .filter(user::Column::Email.eq(&request.email))
        .join_rev(sea_orm::JoinType::LeftJoin, user_info::Relation::User.def())
        .select_only()
        .column_as(user::Column::Id, "account_id")
        .column_as(user::Column::Email, "account_email")
        .column_as(user::Column::Password, "account_password")
        .column_as(user_info::Column::Id, "info_id")
        .column_as(user_info::Column::FirstName, "info_first_name")
        .column_as(user_info::Column::MiddleName, "info_middle_name")
        .column_as(user_info::Column::LastName, "info_last_name")
        .column_as(user_info::Column::UserId, "info_user_id")
        .into_model::<ILoginUsersFromQuery>()
        .one(&db.db_connection)
        .await.unwrap();

    // if let None = find_users {
    //     return Response::builder()
    //         .status(StatusCode::UNAUTHORIZED)
    //         .body(().into_response())
    //         .unwrap();
    // }

    // info!("ACCOUNT INFO: {:#?}", find_users)

    // println!("{:?}", find_users)

    if let None = find_users {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(().into_response())
            .unwrap();
    }

    let users = find_users.unwrap();

    
    if let Some(_) = request.verify_account(&users.account_password) {

        let users_token = users.jwt_signed_with_key("PASSWORD_TEST");
        
        let mut redis = Redis::new(users_token[1].to_string(), db.redis_connection.clone());
        redis.stored_value(&users_token[0], Some(604_800)).await.unwrap();

        let make_response_body = ResponseLogin {
            jwt_token: users_token[0].to_string(),
            session_id: users_token[1].to_string(),
            info_id: users.info_id,
            info_first_name: users.info_first_name,
            info_middle_name: users.info_middle_name,
            info_last_name: users.info_last_name,
        };


        return Response::builder()
            .status(StatusCode::CREATED)
            .header(header::COOKIE, format!("Authorization={}", users_token[0].to_string()))
            .header(header::COOKIE, format!("_sid={}", users_token[1].to_string()))
            .body(Json(&make_response_body).into_response())
            .unwrap();
    }

    Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(().into_response())
            .unwrap()
}

pub async fn delete_user(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteUserById>
) -> impl IntoResponse {

    let find_user = user::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if find_user.is_none() {
        return (StatusCode::NOT_FOUND, format!(""));
    }

    let mut make_active_model = find_user.unwrap().into_active_model();

    make_active_model.is_delete = Set(true);

    make_active_model.update(&db.db_connection).await.unwrap();

    Redis::new("get_all_users".to_string(), db.redis_connection.clone())
            .remove_multi_value(vec![
                "get_all_users_with_roles", 
                "get_all_account_info", 
                format!("get_users_all_info_by_id_{}", request.id).as_str()
            ]).await.unwrap();

    (StatusCode::CREATED, format!("OK"))
}