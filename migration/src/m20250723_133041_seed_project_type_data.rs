use entity::settings::settings_type;
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
            settings_type::ActiveModel {
                name: Set("Local Road-Road Concreting".to_string()),
                ..Default::default()
            },
            settings_type::ActiveModel {
                name: Set("Bridge".to_string()),
                ..Default::default()
            },
            settings_type::ActiveModel {
                name: Set("Public Market".to_string()),
                ..Default::default()
            },
            settings_type::ActiveModel {
                name: Set("Slaughterhouse".to_string()),
                ..Default::default()
            },
            settings_type::ActiveModel {
                name: Set("Multi-purpose Building/Hall".to_string()),
                ..Default::default()
            },
            settings_type::ActiveModel {
                name: Set("Multi-purpose Pavement".to_string()),
                ..Default::default()
            },
            settings_type::ActiveModel {
                name: Set("Drainage Canal".to_string()),
                ..Default::default()
            },
            settings_type::ActiveModel {
                name: Set("Sea Wall/River Control".to_string()),
                ..Default::default()
            },
            settings_type::ActiveModel {
                name: Set("Water System".to_string()),
                ..Default::default()
            },
            settings_type::ActiveModel {
                name: Set("Evacuation Center".to_string()),
                ..Default::default()
            }
       ];

        settings_type::Entity::insert_many(make_add).exec(db).await.unwrap();

       Ok(())
    }
}