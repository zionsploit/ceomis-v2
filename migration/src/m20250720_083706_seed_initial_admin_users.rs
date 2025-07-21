use entity::user;
use sea_orm::{prelude::*, ActiveValue::Set};
use sea_orm_migration::prelude::*;
use services::helpers::Helpers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        let db = manager.get_connection();

        let create_account = user::ActiveModel {
            email: Set("developer@gmail.com".to_string()),
            password: Set(Helpers::string_to_sha256("password")),
            user_roles_id: Set(5),
            ..Default::default()
        };

        create_account.save(db).await.unwrap();

        Ok(())
    }
}