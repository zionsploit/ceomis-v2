use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use ts_rs::TS;

use crate::helpers::Helpers;


#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct RequestAddUser {
    pub email: String,
    pub password: String,
    pub user_role: i32
}

impl RequestAddUser {
    pub fn hash_password(&self) -> String {
        let mut hashes = Sha256::new();

        hashes.update(format!("{}", self.password).as_bytes());
        
        let finalized_hash = hashes.finalize();

        hex::encode(finalized_hash)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestUserInfo {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub user_id: i32
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct RequestAddUpdateUserInfoById {
    pub id: Option<i32>,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub user_id: i32
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct RequestUserLogin {
    pub email: String,
    pub password: String,
}

impl RequestUserLogin {
    pub fn verify_account (&self, hash_password: &str) -> Option<bool> {
        
        if self.password.is_empty() {
            return None;
        }

        let hashed_password = {
            let mut hashes = Sha256::new();
            hashes.update(self.password.as_bytes());
            let finalized_hash = hashes.finalize();
            hex::encode(finalized_hash)
        };

        if hashed_password.eq(hash_password) {
            return Some(true);
        }

        None
    }
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct RequestUpdateUser {
    pub id: i32,
    pub email: String,
    pub password: Option<String>,
    pub user_role: i32,
}

impl RequestUpdateUser {
    pub fn hash_password(&self) -> String {
        Helpers::string_to_sha256(&self.password.clone().unwrap())
    }
}

#[derive(Deserialize, Clone, TS)]
#[ts(export, export_to = "../../../client/src/types/Users.ts")]
pub struct RequestDeleteUserById {
    pub id: i32
}