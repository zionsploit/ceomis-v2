use entity::settings::settings_categories;
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
            settings_categories::ActiveModel {
                name: Set("Construction".to_string()),
                ..Default::default()
            },
            settings_categories::ActiveModel {
                name: Set("Concreting".to_string()),
                ..Default::default()
            },
            settings_categories::ActiveModel {
                name: Set("Repair".to_string()),
                ..Default::default()
            },
            settings_categories::ActiveModel {
                name: Set("Improvement".to_string()),
                ..Default::default()
            },
            settings_categories::ActiveModel {
                name: Set("School Building".to_string()),
                ..Default::default()
            },
        ];

        settings_categories::Entity::insert_many(make_add).exec(db).await.unwrap();

        Ok(())
    }
}