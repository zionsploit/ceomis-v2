use entity::settings::settings_sector;
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        let make_add = [
            settings_sector::ActiveModel {
                name: Set("Economic sector".to_string()),
                ..Default::default()
            },
            settings_sector::ActiveModel {
                name: Set("Social sector".to_string()),
                ..Default::default()
            },
            settings_sector::ActiveModel {
                name: Set("Infrastructure".to_string()),
                ..Default::default()
            },
            settings_sector::ActiveModel {
                name: Set("Environment & Natural Resources".to_string()),
                ..Default::default()
            },
            settings_sector::ActiveModel {
                name: Set("Institutional".to_string()),
                ..Default::default()
            },
        ];

        settings_sector::Entity::insert_many(make_add).exec(db).await.unwrap();

        Ok(())
    }
}