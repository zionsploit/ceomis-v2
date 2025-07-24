use entity::settings::settings_incharge;
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
            settings_incharge::ActiveModel {
                name: Set("City (Admin)".to_string()),
                ..Default::default()
            },
            settings_incharge::ActiveModel {
                name: Set("City (Contract)".to_string()),
                ..Default::default()
            },
            settings_incharge::ActiveModel {
                name: Set("Barangay".to_string()),
                ..Default::default()
            },
        ];

        settings_incharge::Entity::insert_many(make_add).exec(db).await.unwrap();

        Ok(())
    }
}