use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Serialize, TS, Deserialize)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct ResponseLogin {
    pub jwt_token: String,
    pub session_id: String
}

#[derive(Deserialize, Serialize, FromQueryResult)]
pub struct ResponseUsers {
    pub id: i32,
    pub email: String
}

#[derive(Deserialize, Serialize, FromQueryResult)]
pub struct ResponseAccountInfo {
    pub id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub user_id: i32,
}

#[derive(Deserialize, Serialize, FromQueryResult, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct ResponseUserRoles {
    pub id: i32,
    pub name: String
}


#[derive(Deserialize, Serialize, FromQueryResult, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct ResponseUsersWithRoles {
    pub user_id: i32,
    pub email: String,
    pub user_roles_id: i32,
     #[sea_orm(nested)]
    pub user_roles: Option<ResponseUserRoles>
}

// RESPONSE USERS WITH FULL INFO
#[derive(Deserialize, Serialize, FromQueryResult, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct ResponseUsersRolesForUsersWithFullInfo {
    pub role_id: i32,
    pub role_name: String
}

#[derive(Deserialize, Serialize, FromQueryResult, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct ResponseAccountInfoForUsersWithFullInfo {
    pub user_info_id: i32,
    pub user_info_first_name: String,
    pub user_info_middle_name: String,
    pub user_info_last_name: String,
}

#[derive(Deserialize, Serialize, FromQueryResult, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct ResponseUsersWithFullInfo {
    pub user_id: i32,
    pub user_email: String,
    #[sea_orm(nested)]
    pub user_roles: Option<ResponseUsersRolesForUsersWithFullInfo>,
    #[sea_orm(nested)]
    pub user_info: Option<ResponseAccountInfoForUsersWithFullInfo>,
}